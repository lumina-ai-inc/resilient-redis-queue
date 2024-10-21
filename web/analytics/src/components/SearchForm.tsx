import { useState, useEffect } from "react";
import { SearchFormProps } from "../models/components.model";

export default function SearchForm({
  onSearch,
  defaultValue = "main*",
}: SearchFormProps) {
  const [searchTerm, setSearchTerm] = useState(defaultValue);

  useEffect(() => {
    setSearchTerm(defaultValue);
  }, [defaultValue]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSearch(searchTerm.trim());
  };

  return (
    <form onSubmit={handleSubmit} className="search-form">
      <label htmlFor="search">Search Queues:</label>
      <input
        type="text"
        id="search"
        value={searchTerm}
        onChange={(e) => setSearchTerm(e.target.value)}
        placeholder="Enter search term"
      />
      <button type="submit">Search</button>
    </form>
  );
}
