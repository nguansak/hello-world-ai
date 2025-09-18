import { useState } from 'react'
import Datepicker from './components/Datepicker'

function App() {
  const [activeTab, setActiveTab] = useState('Dashboard')

  const sidebarItems = [
    { name: 'Dashboard', icon: '📊', active: true },
    { name: 'Courses', icon: '📚', active: false },
    { name: 'Chats', icon: '💬', active: false },
    { name: 'Grades', icon: '📝', active: false },
    { name: 'Schedule', icon: '📅', active: false },
    { name: 'Settings', icon: '⚙️', active: false }
  ]

  const newCourses = [
    { 
      title: 'Geography', 
      lessons: 12, 
      color: 'bg-orange-400',
      icon: '🗺️',
      students: ['/api/placeholder/24/24', '/api/placeholder/24/24', '/api/placeholder/24/24']
    },
    { 
      title: 'JavaScript Course', 
      lessons: 15, 
      color: 'bg-purple-400',
      icon: '</>', 
      students: ['/api/placeholder/24/24', '/api/placeholder/24/24', '/api/placeholder/24/24']
    },
    { 
      title: 'Photography Course', 
      lessons: 8, 
      color: 'bg-blue-400',
      icon: '📷',
      students: ['/api/placeholder/24/24', '/api/placeholder/24/24', '/api/placeholder/24/24']
    }
  ]

  const myCourses = [
    { icon: '🎨', title: 'Web Design', lessons: 10, date: 'May 12', rate: 4.8, level: 'Elementary' },
    { icon: '</>', title: 'Development Basics', lessons: 8, date: 'May 14', rate: 4.4, level: 'Intermediate' },
    { icon: '🐍', title: 'Data with Python', lessons: 4, date: 'May 17', rate: 4.6, level: 'Intermediate' },
    { icon: '📊', title: 'Html Basics', lessons: 12, date: 'May 26', rate: 4.7, level: 'Elementary' },
    { icon: '⚡', title: 'JavaScript', lessons: null, date: 'May 30', rate: 4.9, level: 'Elementary' }
  ]

  return (
    <div className="bg-purple-100 min-h-screen flex">
      {/* Sidebar */}
      <div className="w-64 bg-white rounded-r-3xl shadow-lg p-6">
        {/* Logo */}
        <div className="flex items-center mb-8">
          <div className="w-8 h-8 bg-purple-500 rounded-lg flex items-center justify-center mr-3">
            <span className="text-white font-bold">A</span>
          </div>
          <span className="text-lg font-semibold text-gray-800">Academy</span>
        </div>

        {/* Navigation */}
        <nav className="space-y-2">
          {sidebarItems.map((item) => (
            <div
              key={item.name}
              className={`flex items-center px-4 py-3 rounded-xl cursor-pointer transition-colors ${
                item.name === activeTab 
                  ? 'bg-purple-100 text-purple-700' 
                  : 'text-gray-600 hover:bg-gray-50'
              }`}
              onClick={() => setActiveTab(item.name)}
            >
              <span className="mr-3">{item.icon}</span>
              <span className="font-medium">{item.name}</span>
            </div>
          ))}
        </nav>

        {/* Premium Subscription */}
        <div className="mt-8 bg-purple-50 rounded-xl p-4">
          <div className="text-center mb-4">
            <div className="w-16 h-16 bg-purple-200 rounded-full mx-auto mb-3 flex items-center justify-center">
              👨‍💻
            </div>
            <h3 className="font-semibold text-gray-800">Premium subscription</h3>
            <p className="text-sm text-gray-600">Buy Premium and get access to new courses</p>
          </div>
          <button className="w-full bg-purple-600 text-white py-2 rounded-lg hover:bg-purple-700 transition-colors">
            More detailed
          </button>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 p-8">
        {/* Header */}
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-2xl font-bold text-gray-800">Dashboard</h1>
          <div className="flex items-center space-x-4">
            <div className="relative">
              <input 
                type="text" 
                placeholder="Search"
                className="bg-white rounded-lg px-4 py-2 pr-10 border border-gray-200 focus:outline-none focus:ring-2 focus:ring-purple-500"
              />
              <span className="absolute right-3 top-2.5 text-gray-400">🔍</span>
            </div>
            <div className="w-8 h-8 bg-gray-200 rounded-lg flex items-center justify-center cursor-pointer">
              🔔
            </div>
            <span className="font-medium text-gray-700">Profile</span>
            <div className="w-8 h-8 bg-gray-200 rounded-lg flex items-center justify-center cursor-pointer">
              ✏️
            </div>
          </div>
        </div>

        <div className="flex gap-8">
          {/* Left Column */}
          <div className="flex-1">
            {/* New Courses */}
            <div className="mb-8">
              <h2 className="text-xl font-semibold text-gray-800 mb-4">New Courses</h2>
              <div className="grid grid-cols-3 gap-4">
                {newCourses.map((course, index) => (
                  <div key={index} className="bg-white rounded-xl p-6 shadow-sm">
                    <div className={`w-16 h-16 ${course.color} rounded-xl flex items-center justify-center mb-4`}>
                      <span className="text-2xl">{course.icon}</span>
                    </div>
                    <h3 className="font-semibold text-gray-800 mb-1">{course.title}</h3>
                    <p className="text-sm text-gray-600 mb-4">{course.lessons} lessons</p>
                    <div className="flex justify-between items-center">
                      <div className="flex -space-x-2">
                        {course.students.map((student, idx) => (
                          <div key={idx} className="w-6 h-6 bg-gray-300 rounded-full border-2 border-white"></div>
                        ))}
                      </div>
                      <div className={`w-8 h-8 ${course.color} rounded-lg flex items-center justify-center cursor-pointer`}>
                        <span className="text-white">→</span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* My Courses */}
            <div>
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-xl font-semibold text-gray-800">My Courses</h2>
                <a href="#" className="text-purple-600 text-sm hover:underline">View All</a>
              </div>
              <div className="bg-white rounded-xl overflow-hidden shadow-sm">
                <table className="w-full">
                  <thead className="bg-gray-50">
                    <tr>
                      <th className="text-left p-4 text-sm font-medium text-gray-600">Course name</th>
                      <th className="text-left p-4 text-sm font-medium text-gray-600">Start</th>
                      <th className="text-left p-4 text-sm font-medium text-gray-600">Rate</th>
                      <th className="text-left p-4 text-sm font-medium text-gray-600">Level</th>
                    </tr>
                  </thead>
                  <tbody>
                    {myCourses.map((course, index) => (
                      <tr key={index} className="border-t border-gray-100">
                        <td className="p-4">
                          <div className="flex items-center">
                            <span className="text-2xl mr-3">{course.icon}</span>
                            <div>
                              <div className="font-medium text-gray-800">{course.title}</div>
                              {course.lessons && (
                                <div className="text-sm text-gray-600">{course.lessons} lessons</div>
                              )}
                            </div>
                          </div>
                        </td>
                        <td className="p-4 text-gray-600">{course.date}</td>
                        <td className="p-4 text-gray-600">{course.rate}</td>
                        <td className="p-4">
                          <span className="px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm">
                            {course.level}
                          </span>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>

          {/* Right Column */}
          <div className="w-80">
            {/* Profile */}
            <div className="bg-white rounded-xl p-6 mb-6 shadow-sm">
              <div className="text-center mb-4">
                <div className="w-16 h-16 bg-orange-200 rounded-full mx-auto mb-3 flex items-center justify-center">
                  👨‍🎓
                </div>
                <h3 className="font-semibold text-gray-800">Esther Howard</h3>
                <p className="text-sm text-gray-600">Elementary</p>
              </div>
            </div>

            {/* Calendar */}
            <div className="bg-white rounded-xl p-6 mb-6 shadow-sm">
              <div className="flex justify-between items-center mb-4">
                <h3 className="font-semibold text-gray-800">May 2022</h3>
                <div className="flex space-x-2">
                  <button className="w-6 h-6 text-gray-400 hover:text-gray-600">←</button>
                  <button className="w-6 h-6 text-gray-400 hover:text-gray-600">→</button>
                </div>
              </div>
              <Datepicker />
            </div>

            {/* Homework Progress */}
            <div className="bg-white rounded-xl p-6 shadow-sm">
              <h3 className="font-semibold text-gray-800 mb-4">Homework progress</h3>
              <div className="space-y-4">
                <div className="flex items-center justify-between p-4 bg-purple-600 rounded-xl text-white">
                  <div className="flex items-center">
                    <div className="w-8 h-8 bg-white bg-opacity-20 rounded-lg flex items-center justify-center mr-3">
                      <span>CSS</span>
                    </div>
                    <div>
                      <div className="font-medium">Styling with CSS</div>
                      <div className="text-sm opacity-80">5 tasks</div>
                    </div>
                  </div>
                  <span>→</span>
                </div>
                
                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-xl">
                  <div className="flex items-center">
                    <div className="w-8 h-8 bg-gray-200 rounded-lg flex items-center justify-center mr-3">
                      <span className="text-xs">60%</span>
                    </div>
                    <div>
                      <div className="font-medium text-gray-800">Basics of programming</div>
                      <div className="text-sm text-gray-600">18 tasks</div>
                    </div>
                  </div>
                  <span>→</span>
                </div>

                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-xl">
                  <div className="flex items-center">
                    <div className="w-8 h-8 bg-gray-200 rounded-lg flex items-center justify-center mr-3">
                      <span className="text-xs">25%</span>
                    </div>
                    <div>
                      <div className="font-medium text-gray-800">Learn to Program in Java</div>
                      <div className="text-sm text-gray-600">30 tasks</div>
                    </div>
                  </div>
                  <span>→</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default App
