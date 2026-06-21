import type { Course } from '../types';

function EditButton({ course, editPanel }: { course: Course, editPanel: (course: Course) => void }) {
  return <button onClick={() => editPanel(course)}>🖊</button>;
}

export default EditButton;