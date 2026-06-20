import type { Course } from '../types';

export default function EditButton({ course, editPanel }: { course: Course, editPanel: (course: Course) => void }) {
  return <button onClick={() => editPanel(course)}>🖊</button>;
}
