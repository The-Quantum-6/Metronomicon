import type { Course } from '../types';

type Props = {
  course: Course | null;
  isEditing: boolean;
  setIsEditing: (v: boolean) => void;
  newCourse: (v: Course | null) => void;
};

function CoursePanel({ course, isEditing, setIsEditing, newCourse }: Props) {

  async function handleSubmit(e: React.SyntheticEvent<HTMLFormElement>) {
  e.preventDefault();
  const formData = new FormData(e.currentTarget);

    const body = JSON.stringify({
      name: formData.get('subjectName'),
      content: formData.get('description') || null,
      code: formData.get('subjectCode'),
    });

    try{
      if (course) {
        const res = await fetch(`/courses/${course.id}`, {
          method: 'PUT',
          body,
          headers: { 'Content-Type': 'application/json' },
        });
        if (!res.ok)  throw new Error('Kunne ikke oppdatere kurset');
      } else {
        const res = await fetch('/courses/create', {
          method: 'POST',
          body,
          headers: { 'Content-Type': 'application/json' },
        });
        if (!res.ok) throw new Error('Kunne ikke opprette kurs');
      }
      setIsEditing(false);
      newCourse(null);
    }
    catch (err) {
      console.error(err);
    }
  }

  return (
    <>
      {isEditing ? (
        <>
          <h2>Course info</h2>
          <form onSubmit={handleSubmit}>
            <label>
              Subject Code: <input name="subjectCode" defaultValue={course?.code} required />
            </label>
            <label>
              Subject Name: <input name="subjectName" defaultValue={course?.name} required />
            </label>
            <br />
            <br />
            <label>
              Description
              <br /><textarea name="description" defaultValue={course?.content ?? undefined} rows={4} cols={40} />
            </label>
            <hr />
            <button onClick={() => setIsEditing(false)}>cancel</button>
            <button type="submit">submit</button>
          </form>
        </>
      ) : (
        <button onClick={() => { newCourse(null); setIsEditing(true); }}>+</button>
      )}
    </>
  );
}

export default CoursePanel;