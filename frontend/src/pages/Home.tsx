import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";

type Course = {
  id: string;
  name: string;
  code: string;
};

function Home() {
  const [courses, setCourses] = useState<Course[] | null>(null);

  useEffect(() => {
    fetch(apiUrl("courses"))
      .then((r) => r.json())
      .then((data) => setCourses(data))
      .catch(() => setCourses([]));
  }, []);

  return (
    <div className="min-h-screen bg-gray-800 text-white">
      <Navbar />
      <header className="container mx-auto p-6 border-b border-gray-700">
        <h1 className="text-4xl font-bold text-purple-400">Metronomicon</h1>
        <p className="text-gray-400 mt-2">A simple course prototype</p>
      </header>

      <main className="container mx-auto p-6">
        <h2 className="text-2xl mb-6 text-white">Courses</h2>
        {courses === null ? (
          <div className="text-gray-400">Loading...</div>
        ) : courses.length === 0 ? (
          <div className="text-gray-400">No courses yet.</div>
        ) : (
          <ul className="space-y-3">
            {courses.map((c) => (
              <li key={c.id}>
                <Link to={`/courses/${c.id}`} className="text-purple-400 hover:text-purple-300 transition block p-3 rounded hover:bg-gray-700">
                  {c.name} <span className="text-gray-500">({c.code})</span>
                </Link>
              </li>
            ))}
          </ul>
        )}
      </main>
    </div>
  );
}

export default Home;
