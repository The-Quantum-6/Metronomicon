import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { apiUrl } from "../config";

type Course = { id: string; name: string; code: string };

export default function Admin() {
  const [courses, setCourses] = useState<Course[] | null>(null);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();

  const fetchCourses = () => {
    setLoading(true);
    fetch(apiUrl("courses"))
      .then((r) => r.json())
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
        fetchCourses();
      })
      .catch(() => alert("Delete failed"));
  };

  return (
    <div className="min-h-screen bg-gray-800 text-white">
      <Navbar />
      <main className="container mx-auto p-6">
        <div className="flex items-center justify-between mb-6">
          <h1 className="text-2xl font-bold text-purple-400">Admin — Courses</h1>
          <button onClick={() => navigate("/admin/create")} className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded transition">Create New Course</button>
        </div>

        {loading ? (
          <div className="text-gray-400">Loading...</div>
        ) : courses && courses.length > 0 ? (
          <div className="overflow-x-auto bg-gray-900 rounded border border-gray-700">
            <table className="min-w-full">
              <thead className="bg-gray-700 border-b border-gray-600">
                <tr>
                  <th className="text-left p-3 text-purple-400">Name</th>
                  <th className="text-left p-3 text-purple-400">Code</th>
                  <th className="text-left p-3 text-purple-400">Actions</th>
                </tr>
              </thead>
              <tbody>
                {courses.map((c) => (
                  <tr key={c.id} className="border-t border-gray-700 hover:bg-gray-800">
                    <td className="p-3">
                      <Link to={`/courses/${c.id}`} className="text-purple-400 hover:text-purple-300 transition">{c.name}</Link>
                    </td>
                    <td className="p-3 text-gray-300">{c.code}</td>
                    <td className="p-3">
                      <button onClick={() => handleDelete(c.id)} className="bg-red-600 hover:bg-red-700 text-white px-3 py-1 rounded text-sm transition">Delete</button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <div className="text-gray-400">No courses.</div>
        )}
      </main>
    </div>
  );
}
