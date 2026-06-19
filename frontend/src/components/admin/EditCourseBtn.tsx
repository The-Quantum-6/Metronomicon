import type { Course } from '../types';

export default function EditButton({ course, editPanel }: { course: Course, editPanel: (course: Course) => void }) {
  async function handleClick() {
    const res = await fetch(`/admin/course/${course.id}`);
    const data = await res.json();
    editPanel(data);
  }
  return <button onClick={handleClick}>🖊</button>;
}
