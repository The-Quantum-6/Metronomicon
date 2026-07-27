import { useState } from "react";
import type { ContributeMode } from "../Contribute";

interface Props {
  courseId: string;
  mode: ContributeMode;
  onCancel: () => void;
}

// TODO: use courseId and mode when file upload is ready
// (direct: POST /files then /resources Create; propose: /contributions with File::AddResource)
export default function ResourceForm({onCancel }: Props) {
  const [title, setTitle] = useState("");
  const [file, setFile] = useState<File | null>(null);
  const [submitted, setSubmitted] = useState(false);

  function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault();
    // TODO: implement file upload when ready
    console.log("submit resource", title, file);
    setSubmitted(true);
  }

  if (submitted) {
    return (
      <div className="text-center py-6 bg-[#F4F2EB] rounded-lg">
        <h3 className="text-xl font-semibold text-[#1A1F3A] mb-2">Thank you for your contribution!</h3>
        <p className="text-[#6B6B5A] mb-6">Your file or document has been submitted and is waiting for review.</p>
        <button type="button" onClick={onCancel} className="px-4 py-2 bg-[#1A1F3A] text-lg text-white rounded-lg hover:opacity-90">
          Back to course
        </button>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-5">
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">Title</label>
        <input type="text" placeholder="Example: Lecture notes week 1" value={title}
          onChange={(e) => setTitle(e.target.value)} required
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
      </div>
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">File</label>
        {file ? (
          <div className="flex items-center justify-center border border-[#6B6B5A] rounded-lg px-4 py-3">
            <div className="flex items-center w-full">
              <p className="text-lg text-[#6B6B5A]">{file.name}</p>
              <button onClick={() => setFile(null)}
              className="ml-auto text-[#6B6B5A] hover:text-red-600 transition-colors">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 30 30" className="w-6 h-6" fill="currentColor"><path d="M 14.984375 2.4863281 A 1.0001 1.0001 0 0 0 14 3.5 L 14 4 L 8.5 4 A 1.0001 1.0001 0 0 0 7.4863281 5 L 6 5 A 1.0001 1.0001 0 1 0 6 7 L 24 7 A 1.0001 1.0001 0 1 0 24 5 L 22.513672 5 A 1.0001 1.0001 0 0 0 21.5 4 L 16 4 L 16 3.5 A 1.0001 1.0001 0 0 0 14.984375 2.4863281 z M 6 9 L 7.7929688 24.234375 C 7.9109687 25.241375 8.7633438 26 9.7773438 26 L 20.222656 26 C 21.236656 26 22.088031 25.241375 22.207031 24.234375 L 24 9 L 6 9 z" />
                </svg>
                </button>
                </div>
                </div>
        ) : (
          <label className="flex flex-col items-center justify-center w-full border-2 border-dashed border-[#6B6B5A] rounded-lg py-8 cursor-pointer hover:bg-gray-50 transition-colors">
            <svg className="w-8 h-8 text-[#6B6B5A] mb-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"> <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M12 19V5m0 0-4 4m4-4 4 4M5 19h14"/>
            </svg>
            <p className="text-[#6B6B5A] text-base">Click to upload a file</p>
            <input type="file"
            onChange={(e) => setFile(e.target.files?.[0] ?? null)} // Get the first selected file, or null if none
            required className="hidden"/>
          </label>
        )}
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