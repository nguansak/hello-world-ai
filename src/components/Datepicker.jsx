import { useState, useMemo } from 'react';

const Datepicker = () => {
  const today = new Date();
  const [currentMonth, setCurrentMonth] = useState(today.getMonth());
  const [currentYear, setCurrentYear] = useState(today.getFullYear());
  const [selectedDate, setSelectedDate] = useState(today.getDate());

  const weekdays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

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
        date: prevMonthDays - i,
        isCurrentMonth: false,
        actualDate: new Date(currentYear, currentMonth - 1, prevMonthDays - i)
      });
    }
    
    // current month days
    const thisMonthDays = daysInMonth(currentYear, currentMonth);
    for (let i = 1; i <= thisMonthDays; i++) {
      days.push({ 
        date: i, 
        isCurrentMonth: true,
        actualDate: new Date(currentYear, currentMonth, i)
      });
    }
    
    // next month days (fill to 6 weeks grid)
    const remainingDays = 42 - days.length;
    for (let i = 1; i <= remainingDays; i++) {
      days.push({ 
        date: i, 
        isCurrentMonth: false,
        actualDate: new Date(currentYear, currentMonth + 1, i)
      });
    }
    
    return days;
  }, [currentYear, currentMonth]);

  const selectDate = (day) => {
    if (day.isCurrentMonth) {
      setSelectedDate(day.date);
    }
  };

  const isSelected = (day) => {
    return day.isCurrentMonth && day.date === selectedDate;
  };

  const isToday = (day) => {
    const todayDate = new Date();
    return day.actualDate.toDateString() === todayDate.toDateString();
  };

  return (
    <div className="w-full">
      {/* Weekdays header */}
      <div className="grid grid-cols-7 gap-1 mb-2">
        {weekdays.map((day) => (
          <div key={day} className="text-center text-xs font-medium text-gray-500 py-2">
            {day.slice(0, 3)}
          </div>
        ))}
      </div>
      
      {/* Calendar grid */}
      <div className="grid grid-cols-7 gap-1">
        {calendarDays.slice(0, 35).map((day, index) => {
          const isSelectedDay = isSelected(day);
          const isTodayDay = isToday(day);
          
          return (
            <button
              key={index}
              onClick={() => selectDate(day)}
              className={`
                h-8 w-8 text-sm rounded-lg flex items-center justify-center transition-colors
                ${!day.isCurrentMonth 
                  ? 'text-gray-300 cursor-default' 
                  : 'text-gray-700 hover:bg-purple-100 cursor-pointer'
                }
                ${isSelectedDay 
                  ? 'bg-purple-600 text-white hover:bg-purple-700' 
                  : ''
                }
                ${isTodayDay && !isSelectedDay 
                  ? 'bg-purple-100 text-purple-600 font-semibold' 
                  : ''
                }
              `}
              disabled={!day.isCurrentMonth}
            >
              {day.date}
            </button>
          );
        })}
      </div>
    </div>
  );
};

export default Datepicker;
