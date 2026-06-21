import { useState, useEffect } from 'react';
import type { Course } from './types';
export default function CourseList() {
  const [courses, setCourses] = useState<Course[]>([]);

  useEffect(() => {
    getCourses().then(data => setCourses(data));
  }, []);

  function getCourses() {
  return fetch('/courses')
    .then(res => {
      if (!res.ok) throw new Error('Kunne ikke hente kurs');
      return res.json();
    })
    .catch(err => {
      console.error(err);
      return [];
    });
}

  return (
    <ul>
      {courses.map(course => (
        <li key={course.id}>
          <span>{course.code} — {course.name}</span>
        </li>
      ))}
    </ul>
  );
}