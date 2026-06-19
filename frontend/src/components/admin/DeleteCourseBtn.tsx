export default function DeleteButton({ id }: { id: number }) {
  async function handleDelete() {
    await fetch(`/admin/course/${id}`, {
      method: 'DELETE',
    });
  }
  return <button onClick={handleDelete}>🗑</button>;
}
