'use client';

import React, { useState } from 'react'; // Import useState
import Scene from '@/components/Scene'; // Import the Scene component

export default function Home() {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [isSubmitted, setIsSubmitted] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    setErrorMessage(null);
    // Ensure isSubmitted is false when retrying after an error, but not if already submitted successfully.
    // However, the form is hidden when isSubmitted is true, so this is mainly for logical clarity.
    if (isSubmitted) setIsSubmitted(false);


    // Basic client-side validation (already present, can be enhanced)
    if (name.trim() === '' || email.trim() === '') {
      setErrorMessage('Name and Email fields are required.');
      setIsLoading(false);
      return;
    }

    try {
      const response = await fetch('/subscriptions', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: new URLSearchParams({ name, email }).toString(),
      });

      if (response.ok) {
        // console.log('Subscription successful:', await response.text()); // .text() consumes the body
        setIsSubmitted(true);
        setName('');
        setEmail('');
        setErrorMessage(null); // Clear any previous errors
      } else {
        const errorText = await response.text(); // Get error message from body
        setIsSubmitted(false); // Ensure form remains visible to show error
        switch (response.status) {
          case 400:
            setErrorMessage(errorText || 'Please check your name and email and try again.');
            break;
          case 500:
            setErrorMessage('Something went wrong on our end. Please try again later.');
            break;
          default:
            setErrorMessage(errorText || `Subscription failed. Status: ${response.status}. Please try again.`);
            break;
        }
        console.error('Subscription failed:', response.status, errorText);
      }
    } catch (err) {
      setIsSubmitted(false); // Ensure form remains visible
      console.error('An error occurred during subscription:', err);
      setErrorMessage('Failed to connect to the server. Please check your internet connection and try again.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-8 sm:p-12 md:p-24 relative bg-gray-100 dark:bg-gray-900">
      {/* Scene component as a full-screen background */}
      <div style={{ position: 'absolute', top: 0, left: 0, width: '100%', height: '100%', zIndex: 0 }}>
        <Scene />
      </div>
      
      {/* Content card */}
      <div className="relative z-10 bg-white/80 dark:bg-black/70 p-8 sm:p-10 md:p-12 rounded-xl shadow-2xl max-w-lg w-full text-center">
        <h1 className="text-3xl sm:text-4xl md:text-5xl font-bold mb-6 text-gray-900 dark:text-white">
          Stay Updated!
        </h1>
        
        {!isSubmitted ? (
          <>
            <p className="text-base sm:text-lg mb-8 text-gray-700 dark:text-gray-300">
              Subscribe to our newsletter for the latest updates and news.
            </p>
            <form onSubmit={handleSubmit} className="flex flex-col gap-4">
              {/* Name Input */}
              <input
                type="text"
                placeholder="Enter your name"
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="flex-grow p-3 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:focus:ring-indigo-400 bg-white/50 dark:bg-gray-700/50 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400"
                required
                disabled={isLoading}
              />
              {/* Email Input */}
              <input
                type="email"
                placeholder="Enter your email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className="flex-grow p-3 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:focus:ring-indigo-400 bg-white/50 dark:bg-gray-700/50 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400"
                required
                disabled={isLoading}
              />
              <button
                type="submit"
                disabled={isLoading}
                className="bg-indigo-600 hover:bg-indigo-700 dark:bg-indigo-500 dark:hover:bg-indigo-600 text-white p-3 rounded-md font-semibold focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 transition-colors duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading ? 'Subscribing...' : 'Subscribe'}
              </button>
            </form>
            {errorMessage && (
              <p className="mt-4 text-red-600 dark:text-red-400 text-sm">
                {errorMessage}
              </p>
            )}
          </>
        ) : (
          <p className="text-lg sm:text-xl text-green-700 dark:text-green-400">
            Thank you for subscribing! We'll keep you updated.
          </p>
        )}
      </div>
    </main>
  );
}
