"""Documentation storage and retrieval system using ChromaDB and RAG."""
import os
import json
from typing import List, Dict, Optional
import chromadb
from chromadb.config import Settings
from sentence_transformers import SentenceTransformer
from langchain.chains import RetrievalQA
from langchain.llms import OpenAI
from langchain.prompts import PromptTemplate

class DocumentationSystem:
    """Manages project documentation storage and retrieval using ChromaDB."""
    
    def __init__(self, persist_dir: str = "data/documentation"):
        """Initialize the documentation system.
        
        Args:
            persist_dir: Directory to store ChromaDB data
        """
        os.makedirs(persist_dir, exist_ok=True)
        
        # Initialize ChromaDB client
        self.client = chromadb.Client(Settings(
            chroma_db_impl="duckdb+parquet",
            persist_directory=persist_dir
        ))
        
        # Initialize embedding model
        self.embedder = SentenceTransformer('all-MiniLM-L6-v2')
        
        # Initialize collection
        self.collection = self.client.get_or_create_collection("project_docs")
        
        # Initialize RAG components
        self.qa_prompt = PromptTemplate(
            template="""You are a helpful assistant for the Biometric Analysis Project. 
            Use the following context to answer the question at the end. If you don't know 
            the answer, say you don't know, don't try to make up an answer.

            Context: {context}

            Question: {question}
            Answer:""",
            input_variables=["context", "question"]
        )
        
    def add_document(self, title: str, content: str, metadata: Optional[Dict] = None):
        """Add a document to the knowledge base.
        
        Args:
            title: Title of the document
            content: Content of the document
            metadata: Optional metadata dictionary
        """
        # Generate embedding
        embedding = self.embedder.encode(content).tolist()
        
        # Add to collection
        self.collection.add(
            documents=[content],
            metadatas=[metadata or {}],
            ids=[title],
            embeddings=[embedding]
        )
        
    def query_documents(self, query: str, n_results: int = 3) -> List[Dict]:
        """Query the documentation system.
        
        Args:
            query: Search query
            n_results: Number of results to return
            
        Returns:
            List of matching documents with metadata
        """
        # Generate query embedding
        query_embedding = self.embedder.encode(query).tolist()
        
        # Query collection
        results = self.collection.query(
            query_embeddings=[query_embedding],
            n_results=n_results
        )
        
        return [
            {
                "document": doc,
                "metadata": meta,
                "score": score
            }
            for doc, meta, score in zip(
                results["documents"][0],
                results["metadatas"][0],
                results["distances"][0]
            )
        ]
        
    def get_rag_response(self, question: str, context: Optional[str] = None) -> str:
        """Get a RAG-based response to a question.
        
        Args:
            question: The question to answer
            context: Optional context to use
            
        Returns:
            Generated response
        """
        # If no context provided, retrieve relevant documents
        if context is None:
            results = self.query_documents(question)
            context = "\n".join([r["document"] for r in results])
            
        # Initialize QA chain
        qa_chain = RetrievalQA.from_chain_type(
            llm=OpenAI(temperature=0),
            chain_type="stuff",
            retriever=None,
            chain_type_kwargs={"prompt": self.qa_prompt}
        )
        
        # Get response
        response = qa_chain.run({
            "context": context,
            "question": question
        })
        
        return response
        
    def save(self):
        """Persist the documentation database."""
        self.client.persist()
