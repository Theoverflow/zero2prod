import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import Home from '../page';

// Mock the Scene component
jest.mock('@/components/Scene', () => {
  return function DummyScene() {
    return <div data-testid="mocked-scene">Mocked Scene</div>;
  };
});

// Mock global fetch
global.fetch = jest.fn();

describe('Home Page', () => {
  beforeEach(() => {
    // Reset fetch mock before each test
    (global.fetch as jest.Mock).mockClear();
    // Clear localStorage or any other global state if necessary
  });

  test('renders static elements correctly', () => {
    render(<Home />);
    expect(screen.getByRole('heading', { name: /Stay Updated!/i })).toBeInTheDocument();
    expect(screen.getByText(/Subscribe to our newsletter for the latest updates and news./i)).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Enter your name')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Enter your email')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Subscribe/i })).toBeInTheDocument();
  });

  test('handles successful newsletter subscription', async () => {
    (global.fetch as jest.Mock).mockResolvedValueOnce({
      ok: true,
      text: async () => 'Subscription successful',
    });

    render(<Home />);

    const nameInput = screen.getByPlaceholderText('Enter your name') as HTMLInputElement;
    const emailInput = screen.getByPlaceholderText('Enter your email') as HTMLInputElement;
    const subscribeButton = screen.getByRole('button', { name: /Subscribe/i });

    fireEvent.change(nameInput, { target: { value: 'Test User' } });
    fireEvent.change(emailInput, { target: { value: 'test@example.com' } });

    fireEvent.click(subscribeButton);

    // Check for loading state
    expect(subscribeButton).toBeDisabled();
    expect(subscribeButton).toHaveTextContent('Subscribing...');
    expect(nameInput).toBeDisabled();
    expect(emailInput).toBeDisabled();


    // Wait for success message and form reset
    await waitFor(() => {
      expect(screen.getByText(/Thank you for subscribing! We'll keep you updated./i)).toBeInTheDocument();
    });
    
    expect(nameInput).toHaveValue('');
    expect(emailInput).toHaveValue('');
    expect(screen.queryByRole('button', { name: /Subscribing.../i })).not.toBeInTheDocument(); // Button text should revert or button disappears
    expect(screen.queryByPlaceholderText('Enter your name')).not.toBeInTheDocument(); // Form hidden
  });

  test('handles API error (400 Bad Request)', async () => {
    const errorMessage = 'Invalid name or email format.';
    (global.fetch as jest.Mock).mockResolvedValueOnce({
      ok: false,
      status: 400,
      text: async () => errorMessage,
    });

    render(<Home />);
    const nameInput = screen.getByPlaceholderText('Enter your name');
    const emailInput = screen.getByPlaceholderText('Enter your email');
    const subscribeButton = screen.getByRole('button', { name: /Subscribe/i });

    fireEvent.change(nameInput, { target: { value: 'Test User' } });
    fireEvent.change(emailInput, { target: { value: 'bademail' } });
    fireEvent.click(subscribeButton);

    // Check for loading state
    expect(subscribeButton).toBeDisabled();
    expect(subscribeButton).toHaveTextContent('Subscribing...');

    // Wait for error message
    await waitFor(() => {
      expect(screen.getByText(errorMessage)).toBeInTheDocument();
    });

    expect(subscribeButton).not.toBeDisabled(); // Button enabled
    expect(subscribeButton).toHaveTextContent('Subscribe'); // Button text reverted
    expect(nameInput).not.toBeDisabled(); // Input enabled
    expect(emailInput).not.toBeDisabled(); // Input enabled
    expect(screen.queryByText(/Thank you for subscribing!/i)).not.toBeInTheDocument();
    expect(nameInput).toHaveValue('Test User'); // Fields not cleared
    expect(emailInput).toHaveValue('bademail');
  });

  test('handles API error (500 Internal Server Error)', async () => {
    (global.fetch as jest.Mock).mockResolvedValueOnce({
      ok: false,
      status: 500,
      text: async () => 'Internal Server Error',
    });

    render(<Home />);
    fireEvent.change(screen.getByPlaceholderText('Enter your name'), { target: { value: 'Test User' } });
    fireEvent.change(screen.getByPlaceholderText('Enter your email'), { target: { value: 'test@example.com' } });
    const subscribeButton = screen.getByRole('button', { name: /Subscribe/i });
    fireEvent.click(subscribeButton);
    
    expect(subscribeButton).toBeDisabled();
    expect(subscribeButton).toHaveTextContent('Subscribing...');

    await waitFor(() => {
      expect(screen.getByText(/Something went wrong on our end. Please try again later./i)).toBeInTheDocument();
    });
    expect(subscribeButton).not.toBeDisabled();
    expect(subscribeButton).toHaveTextContent('Subscribe');
  });

  test('handles network error (failed to fetch)', async () => {
    (global.fetch as jest.Mock).mockRejectedValueOnce(new TypeError('Failed to fetch'));

    render(<Home />);
    fireEvent.change(screen.getByPlaceholderText('Enter your name'), { target: { value: 'Test User' } });
    fireEvent.change(screen.getByPlaceholderText('Enter your email'), { target: { value: 'test@example.com' } });
    const subscribeButton = screen.getByRole('button', { name: /Subscribe/i });
    fireEvent.click(subscribeButton);

    expect(subscribeButton).toBeDisabled();
    expect(subscribeButton).toHaveTextContent('Subscribing...');

    await waitFor(() => {
      expect(screen.getByText(/Failed to connect to the server. Please check your internet connection and try again./i)).toBeInTheDocument();
    });
    expect(subscribeButton).not.toBeDisabled();
    expect(subscribeButton).toHaveTextContent('Subscribe');
  });
  
  test('shows validation error if name or email is empty on submit', async () => {
    render(<Home />);
    const subscribeButton = screen.getByRole('button', { name: /Subscribe/i });

    // Test with empty name
    fireEvent.change(screen.getByPlaceholderText('Enter your email'), { target: { value: 'test@example.com' } });
    fireEvent.click(subscribeButton);
    await waitFor(() => {
        expect(screen.getByText('Name and Email fields are required.')).toBeInTheDocument();
    });
    expect(global.fetch).not.toHaveBeenCalled();

    // Test with empty email
    fireEvent.change(screen.getByPlaceholderText('Enter your name'), { target: { value: 'Test User' } });
    fireEvent.change(screen.getByPlaceholderText('Enter your email'), { target: { value: '' } }); // Clear email
    fireEvent.click(subscribeButton);
    await waitFor(() => {
        expect(screen.getByText('Name and Email fields are required.')).toBeInTheDocument();
    });
    expect(global.fetch).not.toHaveBeenCalled();
    
    // Test with both empty
    fireEvent.change(screen.getByPlaceholderText('Enter your name'), { target: { value: '' } });
    fireEvent.change(screen.getByPlaceholderText('Enter your email'), { target: { value: '' } });
    fireEvent.click(subscribeButton);
    await waitFor(() => {
        expect(screen.getByText('Name and Email fields are required.')).toBeInTheDocument();
    });
    expect(global.fetch).not.toHaveBeenCalled();
  });
});
