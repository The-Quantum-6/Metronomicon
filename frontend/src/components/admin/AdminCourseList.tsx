import { useState, useEffect } from 'react';
import type { Course } from '../types';
import CoursePanel from './CoursePanel';
import EditCourseBtn from './EditCourseBtn';
import DeleteCourseBtn from './DeleteCourseBtn';

export default function AdminCourseList() {
  const [courses, setCourses] = useState<Course[]>([]);
  const [selectedCourse, setSelectedCourse] = useState<Course | null>(null);
  const [isEditing, setIsEditing] = useState(false);

  useEffect(() => { fetchCourses(); }, []);

  async function fetchCourses() {
    try {
      const res = await fetch('/courses');
      if (!res.ok) throw new Error('Kunne ikke hente kurs');
      setCourses(await res.json());
    } catch (err) {
      console.error(err);
    }
  }

  function handleSetIsEditing(v: boolean) {
    setIsEditing(v);
    if (!v) {
      setSelectedCourse(null);
      fetchCourses();
    }
  }

  return (
    <>
      <CoursePanel
        course={selectedCourse}
        isEditing={isEditing}
        setIsEditing={handleSetIsEditing}
        newCourse={setSelectedCourse}
      />
      <ul>
        {courses.map(course => (
          <li key={course.id}>
            <span>{course.code} — {course.name}</span>
            <EditCourseBtn course={course} editPanel={(c) => { setSelectedCourse(c); setIsEditing(true); }} />
            <DeleteCourseBtn id={course.id} onDelete={fetchCourses} />
          </li>
        ))}
      </ul>
    </>
  );
}