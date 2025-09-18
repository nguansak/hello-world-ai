/**
 * @jest-environment jsdom
 */
import { render, screen, fireEvent, within } from '@testing-library/react';
import { describe, it, expect, beforeEach } from 'vitest';
import Datepicker from './Datepicker';

// Mock current date for consistent testing
const mockDate = new Date('2024-03-15'); // Friday, March 15, 2024
const originalDate = Date;

beforeEach(() => {
  global.Date = class extends Date {
    constructor(...args) {
      if (args.length === 0) {
        return mockDate;
      }
      return new originalDate(...args);
    }
    
    static now() {
      return mockDate.getTime();
    }
  };
});

afterEach(() => {
  global.Date = originalDate;
});

describe('Datepicker Component', () => {
  describe('Initial Render', () => {
    it('should render the calendar grid correctly', () => {
      render(<Datepicker />);
      
      // Check if weekday headers are present
      const weekdays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
      weekdays.forEach(day => {
        expect(screen.getByText(day)).toBeInTheDocument();
      });
    });

    it('should render 35 day buttons (5 weeks)', () => {
      render(<Datepicker />);
      
      // Count all day buttons
      const dayButtons = screen.getAllByRole('button');
      expect(dayButtons).toHaveLength(35);
    });

    it('should highlight today\'s date', () => {
      render(<Datepicker />);
      
      // Find today's date button (15th)
      const todayButton = screen.getByRole('button', { name: '15' });
      expect(todayButton).toHaveClass('bg-purple-100', 'text-purple-600', 'font-semibold');
    });

    it('should have current date selected by default', () => {
      render(<Datepicker />);
      
      const selectedButton = screen.getByRole('button', { name: '15' });
      expect(selectedButton).toHaveClass('bg-purple-100', 'text-purple-600');
    });
  });

  describe('Date Selection', () => {
    it('should select a date when clicked', () => {
      render(<Datepicker />);
      
      // Click on day 20
      const dayButton = screen.getByRole('button', { name: '20' });
      fireEvent.click(dayButton);
      
      // Verify the day is selected
      expect(dayButton).toHaveClass('bg-purple-600', 'text-white');
    });

    it('should change selection when different date is clicked', () => {
      render(<Datepicker />);
      
      // First select day 20
      const day20 = screen.getByRole('button', { name: '20' });
      fireEvent.click(day20);
      expect(day20).toHaveClass('bg-purple-600', 'text-white');
      
      // Then select day 25
      const day25 = screen.getByRole('button', { name: '25' });
      fireEvent.click(day25);
      
      // Verify day 25 is selected and day 20 is not
      expect(day25).toHaveClass('bg-purple-600', 'text-white');
      expect(day20).not.toHaveClass('bg-purple-600', 'text-white');
    });

    it('should not select dates from previous/next month', () => {
      render(<Datepicker />);
      
      // Find buttons with light gray text (previous/next month)
      const allButtons = screen.getAllByRole('button');
      const prevNextMonthButtons = allButtons.filter(button => 
        button.classList.contains('text-gray-300')
      );
      
      // Try to click a previous/next month button
      if (prevNextMonthButtons.length > 0) {
        const originalClasses = prevNextMonthButtons[0].className;
        fireEvent.click(prevNextMonthButtons[0]);
        
        // Verify no selection styling was added
        expect(prevNextMonthButtons[0].className).toBe(originalClasses);
        expect(prevNextMonthButtons[0]).not.toHaveClass('bg-purple-600');
      }
    });
  });

  describe('Calendar Logic', () => {
    it('should display correct number of days for March 2024', () => {
      render(<Datepicker />);
      
      // March 2024 has 31 days
      // Find all buttons with current month styling
      const allButtons = screen.getAllByRole('button');
      const currentMonthButtons = allButtons.filter(button => 
        button.classList.contains('text-gray-700') || 
        button.classList.contains('bg-purple-100') ||
        button.classList.contains('bg-purple-600')
      );
      
      expect(currentMonthButtons.length).toBeGreaterThanOrEqual(31);
    });

    it('should show previous month trailing days', () => {
      render(<Datepicker />);
      
      // March 1, 2024 is a Friday, so we should see Feb 25-29 before it
      const allButtons = screen.getAllByRole('button');
      const grayButtons = allButtons.filter(button => 
        button.classList.contains('text-gray-300')
      );
      
      expect(grayButtons.length).toBeGreaterThan(0);
    });

    it('should show next month leading days', () => {
      render(<Datepicker />);
      
      // March 31, 2024 is a Sunday, so we should see some April days after
      const allButtons = screen.getAllByRole('button');
      const grayButtons = allButtons.filter(button => 
        button.classList.contains('text-gray-300')
      );
      
      expect(grayButtons.length).toBeGreaterThan(0);
    });
  });

  describe('Accessibility', () => {
    it('should have proper button roles', () => {
      render(<Datepicker />);
      
      const buttons = screen.getAllByRole('button');
      expect(buttons.length).toBe(35);
      
      // Each button should be properly labeled
      buttons.forEach(button => {
        expect(button).toHaveTextContent(/^\d{1,2}$/);
      });
    });

    it('should have disabled state for non-current month dates', () => {
      render(<Datepicker />);
      
      const allButtons = screen.getAllByRole('button');
      const disabledButtons = allButtons.filter(button => 
        button.disabled || button.classList.contains('cursor-default')
      );
      
      expect(disabledButtons.length).toBeGreaterThan(0);
    });
  });

  describe('Visual States', () => {
    it('should apply hover styles to current month dates', () => {
      render(<Datepicker />);
      
      const currentMonthButton = screen.getByRole('button', { name: '20' });
      
      // Should have hover class
      expect(currentMonthButton).toHaveClass('hover:bg-purple-100');
    });

    it('should not apply hover styles to previous/next month dates', () => {
      render(<Datepicker />);
      
      const allButtons = screen.getAllByRole('button');
      const prevNextButtons = allButtons.filter(button => 
        button.classList.contains('text-gray-300')
      );
      
      prevNextButtons.forEach(button => {
        expect(button).not.toHaveClass('hover:bg-purple-100');
      });
    });

    it('should maintain today styling when not selected', () => {
      render(<Datepicker />);
      
      // Select a different date
      const day20 = screen.getByRole('button', { name: '20' });
      fireEvent.click(day20);
      
      // Today (15th) should still have today styling but not selected styling
      const todayButton = screen.getByRole('button', { name: '15' });
      expect(todayButton).toHaveClass('bg-purple-100', 'text-purple-600', 'font-semibold');
      expect(todayButton).not.toHaveClass('bg-purple-600', 'text-white');
    });
  });

  describe('Component State', () => {
    it('should maintain selected date state across re-renders', () => {
      const { rerender } = render(<Datepicker />);
      
      // Select day 20
      const day20 = screen.getByRole('button', { name: '20' });
      fireEvent.click(day20);
      
      // Re-render component
      rerender(<Datepicker />);
      
      // Verify day 20 is still selected
      const day20AfterRerender = screen.getByRole('button', { name: '20' });
      expect(day20AfterRerender).toHaveClass('bg-purple-600', 'text-white');
    });
  });

  describe('Edge Cases', () => {
    it('should handle clicking the same date twice', () => {
      render(<Datepicker />);
      
      const day20 = screen.getByRole('button', { name: '20' });
      
      // Click twice
      fireEvent.click(day20);
      fireEvent.click(day20);
      
      // Should remain selected
      expect(day20).toHaveClass('bg-purple-600', 'text-white');
    });

    it('should handle rapid successive clicks', () => {
      render(<Datepicker />);
      
      const day20 = screen.getByRole('button', { name: '20' });
      const day25 = screen.getByRole('button', { name: '25' });
      
      // Rapid clicks
      fireEvent.click(day20);
      fireEvent.click(day25);
      fireEvent.click(day20);
      
      // Last clicked should be selected
      expect(day20).toHaveClass('bg-purple-600', 'text-white');
      expect(day25).not.toHaveClass('bg-purple-600', 'text-white');
    });
  });

  describe('CSS Classes', () => {
    it('should apply correct layout classes', () => {
      const { container } = render(<Datepicker />);
      
      // Check for grid layout
      const weekdaysGrid = container.querySelector('.grid-cols-7');
      expect(weekdaysGrid).toBeInTheDocument();
      
      // Check for gap classes
      const gapElements = container.querySelectorAll('.gap-1');
      expect(gapElements.length).toBeGreaterThan(0);
    });

    it('should apply correct sizing classes', () => {
      render(<Datepicker />);
      
      const buttons = screen.getAllByRole('button');
      buttons.forEach(button => {
        expect(button).toHaveClass('h-8', 'w-8', 'text-sm', 'rounded-lg');
      });
    });
  });
});
