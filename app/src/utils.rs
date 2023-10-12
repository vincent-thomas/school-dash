use crate::{
    _schools::{Lesson, ResponseLesson},
    schools::Day,
};

/// Hjälpfunktioner för att göra om från en ResponseLesson till en Lesson
pub fn response_lesson_to_lesson(response_lesson: ResponseLesson) -> Lesson {
    Lesson {
        id: response_lesson.guidId,
        lesson_name: response_lesson.texts[0].clone(),
        teacher: Some(response_lesson.texts[1].clone()),
        start_time: response_lesson.timeStart,
        end_time: response_lesson.timeEnd,
        day: serialize_day(response_lesson.dayOfWeekNumber),
    }
}

/// Gör om en i8 till en Day
pub fn serialize_day(day: i8) -> Day {
    match day {
        1 => Day::Måndag,
        2 => Day::Tisdag,
        3 => Day::Onsdag,
        4 => Day::Torsdag,
        5 => Day::Fredag,
        _ => panic!("Dag är inte mellan 1 och 5"),
    }
}
