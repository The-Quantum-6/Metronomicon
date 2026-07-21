import { useState } from "react";
import { apiUrl } from "../../config";

interface Props {
  courseId: string;
  onCancel: () => void;
}

export default function FaqForm({ courseId, onCancel }: Props) {
  const [question, setQuestion] = useState("");
  const [answer, setAnswer] = useState("");
  const [submitted, setSubmitted] = useState(false);

  async function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault();
    try {
      const response = await fetch(apiUrl("faqs"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ Create: { course_id: courseId, question, answer } }),
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
        <h3 className="text-xl font-semibold text-[#1A1F3A] mb-2">Thank you for your contribution!</h3>
        <p className="text-[#6B6B5A] mb-6">Your FAQ has been submitted and is waiting for review.</p>
        <button type="button" onClick={onCancel} className="px-4 py-2 bg-[#1A1F3A] text-lg text-white rounded-lg hover:opacity-90">
          Back to course
        </button>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-5">
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">Question</label>
        <input type="text" placeholder="Example: Do I need prior experience to take this course?" value={question}
          onChange={(e) => setQuestion(e.target.value)} required
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
      </div>
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">Answer</label>
        <textarea placeholder="No, this course starts from the basics.." value={answer}
          onChange={(e) => setAnswer(e.target.value)} required rows={4}
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none resize-none"/>
      </div>
      <div className="flex justify-end gap-3 pt-2">
        <button type="button" onClick={onCancel} className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100">
          Cancel
        </button>
        <button type="submit" className="px-4 py-2 text-lg bg-[#1A1F3A] text-white rounded-lg hover:opacity-90 disabled:opacity-50">
          Send for review
        </button>
      </div>
    </form>
  );
}