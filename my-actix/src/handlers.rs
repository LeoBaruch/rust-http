use crate::models::Course;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

pub async fn health_check(state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = state.health_check_response.clone();
    let mut count = state.visit_count.lock().unwrap();
    *count += 1;

    let response = format!("{}: {}\r\n", health_check_response, count);

    HttpResponse::Ok().body(response)
}

pub async fn new_course(state: web::Data<AppState>, new_course: web::Json<Course>) -> HttpResponse {
    let mut courses = state.courses.lock().unwrap();
    let course_count = courses
        .iter()
        .filter(|c| c.teacher_id == new_course.teacher_id)
        .count() as u32;
    let course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    courses.push(course);

    println!("Course added: {:?}", courses);

    HttpResponse::Ok().body("Course added.")
}

pub async fn get_course_with_id(
    state: web::Data<AppState>,
    path: web::Path<(u32,)>,
) -> HttpResponse {
    let courses = state.courses.lock().unwrap();
    let course: Vec<&Course> = courses.iter().filter(|c| c.teacher_id == path.0).collect();

    if course.len() > 0 {
        return HttpResponse::Ok().json(course);
    }

    HttpResponse::Ok().json("no course found.".to_string())
}

pub async fn get_course_details(
    state: web::Data<AppState>,
    path: web::Path<(u32, u32)>,
) -> HttpResponse {
    let courses = state.courses.lock().unwrap();
    let course: Vec<&Course> = courses
        .iter()
        .filter(|c| c.teacher_id == path.0 && c.id.unwrap() == path.1)
        .collect();

    if course.len() > 0 {
        return HttpResponse::Ok().json(course.first());
    }

    HttpResponse::Ok().json("no course found.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn new_course_test() {
        let course = web::Json(Course {
            teacher_id: 1,
            id: None,
            name: "Math".to_string(),
            time: None,
        });

        let app_state = web::Data::new(AppState {
            visit_count: Mutex::new(0),
            health_check_response: "I'm OK.".to_string(),
            courses: Mutex::new(vec![]),
        });

        let resq = new_course(app_state, course).await;

        assert_eq!(resq.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state = web::Data::new(AppState {
            visit_count: Mutex::new(0),
            health_check_response: "I'm OK.".to_string(),
            courses: Mutex::new(vec![
                Course {
                    teacher_id: 1,
                    id: Some(1),
                    name: "Math".to_string(),
                    time: Some(Utc::now().naive_utc()),
                },
                Course {
                    teacher_id: 1,
                    id: Some(2),
                    name: "English".to_string(),
                    time: Some(Utc::now().naive_utc()),
                },
            ]),
        });

        let resq = get_course_with_id(app_state, web::Path::from((1,))).await;

        println!("{:?}", resq.body());
        assert_eq!(resq.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_details_success() {
        let app_state = web::Data::new(AppState {
            visit_count: Mutex::new(0),
            health_check_response: "I'm OK.".to_string(),
            courses: Mutex::new(vec![
                Course {
                    teacher_id: 1,
                    id: Some(1),
                    name: "Math".to_string(),
                    time: Some(Utc::now().naive_utc()),
                },
                Course {
                    teacher_id: 1,
                    id: Some(2),
                    name: "English".to_string(),
                    time: Some(Utc::now().naive_utc()),
                },
            ]),
        });

        let resq = get_course_details(app_state, web::Path::from((1, 1))).await;

        println!("{:?}", resq.body());
        assert_eq!(resq.status(), StatusCode::OK);
    }
}
