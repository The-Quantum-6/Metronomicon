import Navbar from "../components/Navbar";
import { useEffect, useState, type FormEvent } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { apiUrl } from "../config";
import ReactQuill from "react-quill-new";
import "react-quill-new/dist/quill.snow.css";

export default function AdminEdit() {
  const { id } = useParams<{ id: string }>();
  const [name, setName] = useState("");
  const [code, setCode] = useState("");
  const [content, setContent] = useState("");
  const [loading, setLoading] = useState(true);
  const [notFound, setNotFound] = useState(false);
  const navigate = useNavigate();

  // Load the existing course and prefill the form.
  useEffect(() => {
    if (!id) return;
    setLoading(true);
    fetch(apiUrl(`courses/${id}`))
      .then((r) => {
        if (!r.ok) throw new Error("not found");
        return r.json();
      })
      .then((data) => {
        setName(data.name ?? "");
        setCode(data.code ?? "");
        setContent(data.content ?? "");
      })
      .catch(() => setNotFound(true))
      .finally(() => setLoading(false));
  }, [id]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const payload = { name, content, code };
    const res = await fetch(apiUrl(`courses/${id}`), {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
    if (res.ok) navigate("/admin");
    else alert("Update failed");
  };

  // Quill toolbar/modules to support headers, lists and common formats
  const modules = {
    toolbar: [
      [{ header: [1, 2, 3, false] }],
      ["bold", "italic", "underline", "strike"],
      [{ list: "ordered" }, { list: "bullet" }],
      ["link", "blockquote", "code-block"],
      ["clean"],
    ],
  };

  const formats = [
    "header",
    "bold",
    "italic",
    "underline",
    "strike",
    "list",
    "link",
    "blockquote",
    "code-block",
  ];

  return (
    <div className="min-h-screen bg-gray-800 text-white">
      <Navbar />
      <main className="container mx-auto p-6">
        <h1 className="text-2xl font-bold mb-6 text-purple-400">Edit Course</h1>
        {loading ? (
          <div className="text-gray-400">Loading...</div>
        ) : notFound ? (
          <div className="text-gray-400">Course not found.</div>
        ) : (
          <form onSubmit={handleSubmit} className="space-y-4 bg-gray-900 p-6 rounded border border-gray-700">
            <div>
              <label className="block text-sm font-medium text-gray-300">Name</label>
              <input className="mt-1 block w-full bg-gray-800 border border-gray-700 rounded p-2 text-white focus:border-purple-600 focus:outline-none" value={name} onChange={(e) => setName(e.target.value)} required />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-300">Code</label>
              <input className="mt-1 block w-full bg-gray-800 border border-gray-700 rounded p-2 text-white focus:border-purple-600 focus:outline-none" value={code} onChange={(e) => setCode(e.target.value)} required />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">Content (rich text)</label>
              <ReactQuill theme="snow" value={content} onChange={setContent} modules={modules} formats={formats} />
              <p className="text-sm text-gray-500 mt-2">Preview (raw HTML stored):</p>
              <pre className="mt-2 p-3 bg-gray-700 rounded max-h-40 overflow-auto text-xs text-gray-300">{content}</pre>
            </div>

            <div className="flex gap-3">
              <button type="submit" className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded transition">Save Changes</button>
              <button type="button" onClick={() => navigate("/admin")} className="bg-gray-700 hover:bg-gray-600 text-white px-4 py-2 rounded transition">Cancel</button>
            </div>
          </form>
        )}
      </main>
    </div>
  );
}
