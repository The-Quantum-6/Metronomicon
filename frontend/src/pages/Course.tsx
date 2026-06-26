import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { apiUrl } from "../config";

type Course = {
  id: string;
  name: string;
  code: string;
  content?: string | null;
};

export default function CoursePage() {
  const { id } = useParams<{ id: string }>();
  const [course, setCourse] = useState<Course | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!id) return;
    setLoading(true);
    fetch(apiUrl(`courses/${id}`))
      .then((r) => {
        if (!r.ok) throw new Error("not found");
        return r.json();
      })
      .then((data) => setCourse(data))
      .catch(() => setCourse(null))
      .finally(() => setLoading(false));
  }, [id]);

  return (
    <div className="min-h-screen bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto p-6">
        {loading ? (
          <div className="text-text-muted">Loading...</div>
        ) : course ? (
          <article>
            <h1 className="text-4xl font-bold text-primary mb-2">{course.name}</h1>
            <div className="text-text-muted mb-6 text-sm">{course.code}</div>
            <div className="ql-editor" dangerouslySetInnerHTML={{ __html: course.content ?? "" }} />
          </article>
        ) : (
          <div className="text-text-muted">Course not found.</div>
        )}
      </main>
    </div>
  );
}
