import { useState } from "react";
import { apiUrl } from "../../config";
import type { Faq } from "../../types/courseTypes";

interface Props {
    mode: "create" | "edit" | "delete";
    courseId: string;
    faq?: Faq;
    onCancel: () => void;
}

export default function FaqForm({mode, courseId, faq, onCancel,}: Props) {
    const [question, setQuestion] = useState(faq?.question ?? "");
    const [answer, setAnswer] = useState(faq?.answer ?? "");
    const [comment, setComment] = useState("");
    const [submitted, setSubmitted] = useState(false);
    const [instantContribute, setInstantContribute] = useState(false);
    const [isOfficial, setIsOfficial] = useState(faq?.official == true);

    // TODO: replace with actual auth check
    const isAdmin = true;
    const canInstantContribute = isAdmin && instantContribute;

    async function handleSubmit(e: React.SubmitEvent) {
        e.preventDefault();
        try {
            /*create faq*/
            if (mode === "create") {
                const response = await fetch(
                    apiUrl(canInstantContribute ? "faqs" : "contributions"), {
                        method: "POST",
                        headers: {"Content-Type": "application/json",},
                        credentials: "include",
                        body: JSON.stringify(canInstantContribute ?
                            {
                                //write_text
                                Create: {
                                    course_id: courseId,
                                    question,
                                    answer,
                                },
                            } : {
                                //suggest_text
                                Propose: {
                                    course_id: courseId,
                                    contribution: {
                                        Text: {
                                            AddFaqEntry: {
                                                question,
                                                answer,
                                            },
                                        },
                                    },
                                    comment,
                                },
                            }
                        ),
                    }
                );
                // TODO: Replace this with a better error handling
                if (!response.ok) {
                    const error = await response.text();
                    console.error(error);
                    alert("Failed to create");
                    return;
                }
            }

            //edit faq
            if (mode === "edit") {
                if (canInstantContribute) {
                    const response = await fetch(
                        apiUrl("faqs"), {
                            method: "POST",
                            headers: {
                                "Content-Type": "application/json",
                            },
                            credentials: "include",
                            body: JSON.stringify({
                                //write_text
                                Update: {
                                    faq_id: faq?.faq_id,
                                    course_id: courseId,
                                    question,
                                    answer,
                                    comment,
                                }
                            })
                        }
                    );
                    // TODO: Replace this with a better error handling
                    if (!response.ok) {
                        const error = await response.text();
                        console.error(error);
                        alert("Failed to update faq!");
                        return;
                    };
                }
                else {
                    const response = await fetch(
                        apiUrl("contributions"), {
                            method: "POST",
                            headers: {
                                "Content-Type": "application/json",
                            },
                            credentials: "include",
                            body: JSON.stringify({
                                //suggest_text
                                Propose: {
                                    course_id: courseId,
                                    contribution: {
                                        Text: {
                                            EditFaqEntry: {
                                                faq_id: faq?.faq_id,
                                                question,
                                                answer,
                                            }
                                        }
                                    },
                                    comment
                                }
                            })
                        }
                    );
                    // TODO: Replace this with a better error handling
                    if (!response.ok) {
                        const error = await response.text();
                        console.error(error);
                        alert("Failed to contribute");
                        return;
                    }
                } 
                //write_text
                if (isOfficial && isAdmin) {
                    const response = await fetch(
                        apiUrl("faqs"), {
                            method:"POST",
                            headers:{
                                "Content-Type":"application/json",
                            },
                            credentials:"include",
                            body:JSON.stringify({
                                SetOfficial: {
                                    faq_id: faq?.faq_id,
                                    course_id: courseId,
                                    official: true
                                }
                            })
                        }
                    );
                    // TODO: Replace this with a better error handling
                    if (!response.ok) {
                    const error = await response.text();
                    console.error(error);
                    alert("Failed to create");
                    return;
                }
                }
            }
            //delete faq
            if (mode === "delete") {
                const response = await fetch(
                    apiUrl(canInstantContribute ? "faqs" : "contributions"), {
                        method:"POST",
                        headers:{
                            "Content-Type":"application/json",
                        },
                        credentials:"include",
                        body:JSON.stringify(canInstantContribute ? {
                            //write_text
                                Delete: {
                                    faq_id: faq?.faq_id,
                                    course_id: courseId
                                }
                            } : {
                            //suggest_text
                                Propose: {
                                    course_id:courseId,
                                    contribution: {
                                        Text: {
                                            RemoveFaqEntry: {
                                                faq_id: faq?.faq_id
                                            }
                                        }
                                    },
                                    comment
                                }
                            }
                        )
                    }
                );
                // TODO: Replace this with a better error handling
                    if (!response.ok) {
                        const error = await response.text();
                        console.error(error);
                        alert("Failed to delete faq!");
                        return;
                    };
            }
            setSubmitted(true);
        }

        catch(error){
            console.error(error);
            alert("Something went wrong");
        }
    }

    if (submitted){
        return (
      <div className="text-center py-6 bg-[#F4F2EB] rounded-lg">
        <h3 className="text-xl font-semibold text-[#1A1F3A] mb-2">Thank you for your contribution!</h3>
        <p className="text-[#6B6B5A] mb-6"> {canInstantContribute ? 
        mode === "delete" ? "FAQ deleted successfully" : "FAQ saved successfully" 
        : "Your request has been submitted for review."}</p>
        <button type="button" onClick={onCancel} className="px-4 py-2 bg-[#1A1F3A] text-lg text-white rounded-lg hover:opacity-90">
            Back to course
        </button>
            </div>
        );
    }

    return (
        <form onSubmit={handleSubmit} className="space-y-5">
            <h2 className="text-2xl font-semibold text-[#1A1F3A]">{mode==="create" ? "Add FAQ" : 
            mode==="edit" ? "Edit FAQ" : "Delete FAQ"}</h2>
            {mode !== "delete" ?
            <>
                <div>
                    <label className="block text-lg mb-2 text-[#6B6B5A]">Question</label>
                    <input type="text" placeholder="Example: Do I need prior experience to take this course?" value={question}
                        onChange={(e) => setQuestion(e.target.value)} required
                        className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
                        </div>
                <div>
                    <label className="block text-lg mb-2 text-[#6B6B5A]">Answer</label>
                    <textarea placeholder="No, this course starts from the basics..." value={answer}
                    onChange={(e) => setAnswer(e.target.value)} required rows={4}
                    className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
                </div>
                </>
                :
                <div className="border rounded-lg p-4">{faq?.question}
                </div>
            }
            {!canInstantContribute && (
            <div>
                <label className="block text-lg mb-2 text-[#6B6B5A]">Comment</label> 
                <textarea value={comment} placeholder="Anything we should know?" className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 resize-none focus:outline-none"
                onChange={(e)=>setComment(e.target.value)} required={!canInstantContribute} rows={3}/>
            </div>
            )}
            {isAdmin && (
                <div className="">
                    {
                        mode==="edit" &&
                        <label className="flex items-center gap-2">
                            <input type="checkbox" checked={isOfficial} 
                            onChange={(e) => setIsOfficial(e.target.checked)}/>
                            <span>Set official</span>
                            </label>
                    }
                    <label className="flex items-center gap-2">
                        <input type="checkbox" checked={instantContribute}
                        onChange={(e) => setInstantContribute(e.target.checked)}/>
                        <span>Instant contribute</span>
                    </label>
                </div>
            )}
        <div className="flex justify-end gap-3">
          <button type="button" onClick={onCancel} className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100">
                    Cancel
                </button>
                <button type="submit" className="px-4 py-2 bg-[#1A1F3A] text-white rounded-lg hover:opacity-90 disabled:opacity-50">
                    {mode==="delete" ? "Delete" : canInstantContribute ? "Publish" : "Send for review"}
                </button>
            </div>
            
        </form>
    );
}