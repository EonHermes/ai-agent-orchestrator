import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { TaskForm } from '@/components/TaskForm';
import { TaskStatus } from '@/types';

describe('TaskForm', () => {
  const mockSubmit = jest.fn();

  beforeEach(() => {
    mockSubmit.mockClear();
  });

  it('renders create form by default', () => {
    render(<TaskForm onSubmit={mockSubmit} />);

    expect(screen.getByText('Create New Task')).toBeInTheDocument();
    expect(screen.getByLabelText(/title/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/status/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /create task/i })).toBeInTheDocument();
  });

  it('renders edit form when isEditing is true', () => {
    const task = {
      id: '123',
      title: 'Existing task',
      description: 'Description',
      status: 'in_progress' as TaskStatus,
      priority: 5,
      tags: ['tag1', 'tag2'],
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z',
    };

    render(
      <TaskForm
        onSubmit={mockSubmit}
        initialValues={task}
        isEditing={true}
      />
    );

    expect(screen.getByText('Edit Task')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Existing task')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Description')).toBeInTheDocument();
  });

  it('allows adding tags', async () => {
    render(<TaskForm onSubmit={mockSubmit} />);

    const tagInput = screen.getByPlaceholderText(/add tag/i);
    const addButton = screen.getByRole('button', { name: '+' });

    fireEvent.change(tagInput, { target: { value: 'new-tag' } });
    fireEvent.click(addButton);

    await waitFor(() => {
      expect(screen.getByText('new-tag')).toBeInTheDocument();
    });
  });

  it('removes tags when clicking remove button', async () => {
    render(<TaskForm onSubmit={mockSubmit} />);

    // Add a tag first
    const tagInput = screen.getByPlaceholderText(/add tag/i);
    const addButton = screen.getByRole('button', { name: '+' });
    fireEvent.change(tagInput, { target: { value: 'tag-to-remove' } });
    fireEvent.click(addButton);

    await waitFor(() => {
      expect(screen.getByText('tag-to-remove')).toBeInTheDocument();
    });

    // Remove it
    const removeButton = screen.getByRole('button', { name: 'X' });
    fireEvent.click(removeButton);

    await waitFor(() => {
      expect(screen.queryByText('tag-to-remove')).not.toBeInTheDocument();
    });
  });

  it('disables submit button when title is empty', () => {
    render(<TaskForm onSubmit={mockSubmit} />);

    const submitButton = screen.getByRole('button', { name: /create task/i });
    expect(submitButton).toBeDisabled();
  });

  it('enables submit button when title is filled', () => {
    render(<TaskForm onSubmit={mockSubmit} />);

    const titleInput = screen.getByLabelText(/title/i);
    fireEvent.change(titleInput, { target: { value: 'My task' } });

    const submitButton = screen.getByRole('button', { name: /create task/i });
    expect(submitButton).not.toBeDisabled();
  });
});
