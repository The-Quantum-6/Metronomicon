import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";
import { toast } from "../components/Toast";

type Course = { id: string; name: string; code: string };

export default function Admin() {
  const [courses, setCourses] = useState<Course[] | null>(null);
  const [loading, setLoading] = useState(true);

  const fetchCourses = () => {
    setLoading(true);
    fetch(apiUrl("courses"))
      .then((r) => {
        if (!r.ok) throw new Error(`Failed to load courses (${r.status})`);
        return r.json() as Promise<Course[]>;
      })
      .then((data) => setCourses(data))
      .catch(() => setCourses([]))
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    fetchCourses();
  }, []);

  const handleDelete = (id: string) => {
    if (!confirm("Delete this course?")) return;
    fetch(apiUrl(`courses/${id}`), { method: "DELETE" })
      .then((r) => {
        if (!r.ok) throw new Error("delete failed");
        toast("Course deleted");
        fetchCourses();
      })
      .catch(() => toast("Delete failed", false));
  };

  return (
    <div className="min-h-screen bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto p-6">
        <h1 className="text-2xl font-bold text-primary mb-6">Admin — Courses</h1>

        {loading ? (
          <div className="text-text-muted">Loading...</div>
        ) : courses && courses.length > 0 ? (
          <div className="overflow-x-auto bg-bg rounded border border-surface">
            <table className="min-w-full">
              <thead className="bg-surface border-b border-surface-light">
                <tr>
                  <th className="text-left p-3 text-primary">Name</th>
                  <th className="text-left p-3 text-primary">Code</th>
                  <th className="text-left p-3 text-primary">Actions</th>
                </tr>
              </thead>
              <tbody>
                {courses.map((c) => (
                  <tr key={c.id} className="border-t border-surface hover:bg-surface-dark">
                    <td className="p-3">
                      <Link to={`/courses/${c.id}`} className="text-primary hover:text-primary-light transition">{c.name}</Link>
                    </td>
                    <td className="p-3 text-text-secondary">{c.code}</td>
                    <td className="p-3">
                      <div className="flex gap-2">
                        <Link to={`/admin/edit/${c.id}`} className="bg-accent hover:bg-accent-dark text-white px-3 py-1 rounded text-sm transition">Edit</Link>
                        <button onClick={() => handleDelete(c.id)} className="bg-red-600 hover:bg-red-700 text-white px-3 py-1 rounded text-sm transition">Delete</button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <div className="text-text-muted">No courses.</div>
        )}
      </main>
    </div>
  );
}
