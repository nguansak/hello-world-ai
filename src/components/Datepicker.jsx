import { useState, useMemo } from 'react';

const Datepicker = () => {
  const [selectedDate, setSelectedDate] = useState(null);
  const [showCalendar, setShowCalendar] = useState(false);
  const today = new Date();
  const [currentMonth, setCurrentMonth] = useState(today.getMonth());
  const [currentYear, setCurrentYear] = useState(today.getFullYear());

  const formattedDate = useMemo(() => {
    return selectedDate ? selectedDate.toLocaleDateString() : '';
  }, [selectedDate]);

  const weekdays = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'];

  const daysInMonth = (year, month) => {
    return new Date(year, month + 1, 0).getDate();
  };

  const calendarDays = useMemo(() => {
    const days = [];
    const firstDayOfWeek = new Date(currentYear, currentMonth, 1).getDay();
    
    // previous month days
    const prevMonthDays = daysInMonth(currentYear, currentMonth - 1);
    for (let i = firstDayOfWeek - 1; i >= 0; i--) {
      days.push({
        date: new Date(currentYear, currentMonth - 1, prevMonthDays - i),
        otherMonth: true,
      });
    }
    
    // current month days
    const thisMonthDays = daysInMonth(currentYear, currentMonth);
    for (let i = 1; i <= thisMonthDays; i++) {
      days.push({ 
        date: new Date(currentYear, currentMonth, i), 
        otherMonth: false 
      });
    }
    
    // next month days (fill to 6 weeks grid)
    const nextDays = 42 - days.length;
    for (let i = 1; i <= nextDays; i++) {
      days.push({ 
        date: new Date(currentYear, currentMonth + 1, i), 
        otherMonth: true 
      });
    }
    
    return days;
  }, [currentYear, currentMonth]);

  const currentMonthName = useMemo(() => {
    return new Date(currentYear, currentMonth).toLocaleString('default', {
      month: 'long'
    });
  }, [currentYear, currentMonth]);

  const toggleCalendar = () => {
    setShowCalendar(!showCalendar);
  };

  const prevMonth = () => {
    if (currentMonth === 0) {
      setCurrentMonth(11);
      setCurrentYear(currentYear - 1);
    } else {
      setCurrentMonth(currentMonth - 1);
    }
  };

  const nextMonth = () => {
    if (currentMonth === 11) {
      setCurrentMonth(0);
      setCurrentYear(currentYear + 1);
    } else {
      setCurrentMonth(currentMonth + 1);
    }
  };

  const selectDate = (day) => {
    if (day.otherMonth) return;
    setSelectedDate(day.date);
    setShowCalendar(false);
  };

  const isSelected = (day) => {
    return selectedDate && 
           day.date.toDateString() === selectedDate.toDateString();
  };

  return (
    <div className="relative inline-block">
      <input
        type="text"
        readOnly
        value={formattedDate}
        onClick={toggleCalendar}
        placeholder="Select date"
        className="w-40 p-2 border border-gray-300 rounded cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
      {showCalendar && (
        <div className="absolute top-full left-0 mt-2 bg-white border border-gray-300 rounded shadow-lg z-50 p-4">
          {/* header */}
          <div className="flex justify-between items-center mb-2">
            <button 
              onClick={prevMonth} 
              className="p-1 hover:bg-gray-200 rounded text-white bg-transparent border-none"
            >
              &lt;
            </button>
            <span className="font-medium text-gray-900">
              {currentMonthName} {currentYear}
            </span>
            <button 
              onClick={nextMonth} 
              className="p-1 hover:bg-gray-200 rounded text-white bg-transparent border-none"
            >
              &gt;
            </button>
          </div>
          
          {/* weekdays */}
          <div className="grid grid-cols-7 text-center mb-1">
            {weekdays.map((day) => (
              <span key={day} className="font-semibold text-gray-700">
                {day}
              </span>
            ))}
          </div>
          
          {/* days */}
          <div className="grid grid-cols-7 text-center">
            {calendarDays.map((day, index) => {
              const baseClasses = 'p-2 cursor-pointer rounded-full';
              const otherMonthClasses = day.otherMonth ? 'text-gray-400' : 'text-gray-900';
              const selectedClasses = isSelected(day) ? 'bg-blue-500 text-white' : '';
              const hoverClasses = !day.otherMonth && !isSelected(day) ? 'hover:bg-blue-100' : '';
              
              return (
                <span
                  key={index}
                  onClick={() => selectDate(day)}
                  className={`${baseClasses} ${otherMonthClasses} ${selectedClasses} ${hoverClasses}`}
                >
                  {day.date.getDate()}
                </span>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
};

export default Datepicker;
