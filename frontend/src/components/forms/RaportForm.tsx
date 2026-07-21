import { useState } from "react";
import { apiUrl } from "../../config";

interface Props {
  courseId?: string;
  onCancel: () => void;
}

const courseCategories = [
  "Incorrect information",
  "Copyright violation",
  "Broken link",
  "Other",
]

const siteCategories = [
  "Bug or technical issue",
  "Copyright/sensitive content",
  "Privacy concern",
  "Other",
]

export default function ReportForm({ courseId, onCancel }: Props) {
  const [category, setCategory] = useState("");
  const [description, setDescription] = useState("");
  const [contactEmail, setContactEmail] = useState("");
  const [submitted, setSubmitted] = useState(false);

  const categories = courseId ? courseCategories : siteCategories;

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    try {
      const response = await fetch(apiUrl("reports"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
          Create: {
            course_id: courseId ?? null,
            title: category,
            description: `[${category}] ${description}`,
            contact_email: contactEmail || null,
          },
        }),
      });

      if (response.ok) {
        setSubmitted(true);
      } else {
        // TODO: Replace this with a better error handling
        const error = await response.text();
        console.error(error);
        alert("Something went wrong.");
      }
    } catch (error) {
      console.error(error);
      alert("Could not connect to server.");
    }
  }

  if (submitted) {
    return (
      <div className="text-center py-6 bg-[#F4F2EB] rounded-lg">
        <h3 className="text-xl font-semibold text-[#1A1F3A] mb-2">Report submitted</h3>
        <p className="text-[#6B6B5A] mb-6">Thank you for helping us improve Metronomicon.</p>
        <button
          onClick={onCancel}
          className="px-4 py-2 bg-[#1A1F3A] text-lg text-white rounded-lg hover:opacity-90 transition-colors">Close
        </button>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label className="mb-2 text-[#6B6B5A] block">Category</label>
        <select value={category}
          onChange={(e) => setCategory(e.target.value)} required
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-2 focus:outline-none bg-white">
          <option value="">Select a category...</option>
          {categories.map((c) => (
            <option key={c} value={c}>{c}</option>
          ))}
        </select>
      </div>
      <div>
        <label className="mb-2 text-[#6B6B5A] block">Description</label>
        <textarea placeholder="Describe the issue..." value={description}
          onChange={(e) => setDescription(e.target.value)} required rows={4}
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-2 focus:outline-none resize-none"/>
      </div>
      <div>
        <label className="mb-2 text-[#6B6B5A] block">Contact email <span className="text-xs">(optional)</span></label>
        <input
          type="email" placeholder="your@email.com" value={contactEmail}
          onChange={(e) => setContactEmail(e.target.value)}
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-2 focus:outline-none"/>
      </div>
      <div className="flex justify-end gap-3 pt-2">
        <button onClick={onCancel}
          className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100 transition-colors">
          Cancel
        </button>
        <button type="submit"
          className="px-4 py-2 text-lg bg-[#02061b] text-white rounded-lg hover:opacity-90 transition-colors">
          Send report
        </button>
      </div>
    </form>
  );
}