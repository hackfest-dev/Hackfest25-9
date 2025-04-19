"use client"

import { useState, useEffect } from "react"
import Link from "next/link"
import { usePathname } from "next/navigation"
import { Button } from "@/components/ui/button"
import {
  Wallet,
  LayoutDashboard,
  Layers,
  BarChart3,
  Users,
  Vote,
  Shield,
  CreditCard,
  Bell,
  Menu,
  X,
  User,
  Settings,
  LogOut,
} from "lucide-react"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Badge } from "@/components/ui/badge"
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet"
import { ThemeToggle } from "@/components/theme-toggle"

function getInitials(name: string) {
  return name
    .split(' ')
    .map(word => word[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)
}

function getRandomColor() {
  const colors = [
    'bg-blue-500',
    'bg-green-500',
    'bg-purple-500',
    'bg-pink-500',
    'bg-indigo-500',
    'bg-teal-500',
    'bg-orange-500',
    'bg-red-500'
  ]
  return colors[Math.floor(Math.random() * colors.length)]
}

export function Navbar() {
  const [connected, setConnected] = useState(true)
  const [userProfile, setUserProfile] = useState<any>(null)
  const [avatarColor, setAvatarColor] = useState(getRandomColor())
  const pathname = usePathname()

  useEffect(() => {
    const fetchUserProfile = async () => {
      try {
        const token = localStorage.getItem('token')
        if (!token) return

        // Decode the token to get user info immediately
        const tokenParts = token.split('.')
        if (tokenParts.length === 3) {
          const payload = JSON.parse(atob(tokenParts[1]))
          if (payload.userId) {
            setUserProfile(prev => ({
              ...prev,
              fullName: payload.email?.split('@')[0] || 'User',
              email: payload.email || 'guest@example.com'
            }))
          }
        }

        const response = await fetch('/api/auth/me', {
          headers: {
            'Authorization': `Bearer ${token}`
          }
        })

        if (response.ok) {
          const data = await response.json()
          setUserProfile(data.data)
        }
      } catch (error) {
        console.error('Error fetching user profile:', error)
      }
    }

    fetchUserProfile()
  }, [])

  const navItems = [
    { href: "/dashboard", label: "Dashboard", icon: LayoutDashboard },
    { href: "/tokenize", label: "Tokenize", icon: Layers },
    { href: "/trade", label: "Trade", icon: BarChart3 },
    { href: "/lend", label: "Lend", icon: CreditCard },
    { href: "/community", label: "Community", icon: Users },
    { href: "/governance", label: "Governance", icon: Vote },
    { href: "/trust", label: "Trust", icon: Shield },
  ]

  return (
    <header className="sticky top-0 z-50 w-full border-b border-slate-800 bg-slate-900/95 backdrop-blur supports-[backdrop-filter]:bg-slate-900/75">
      <div className="container flex h-16 items-center justify-between">
        <div className="flex items-center gap-6">
          <Link href="/dashboard" className="flex items-center gap-2">
            <div className="relative size-8 rounded-full bg-primary p-[1px]">
              <div className="absolute inset-0 rounded-full bg-background p-1">
                <div className="h-full w-full rounded-full bg-primary opacity-80"></div>
              </div>
            </div>
            <span className="font-space text-xl font-bold tracking-tight text-white">UnityVault</span>
          </Link>
          <nav className="hidden md:flex">
            <ul className="flex items-center gap-1">
              {navItems.map((item) => {
                const isActive = pathname === item.href
                return (
                  <li key={item.href}>
                    <Link
                      href={item.href}
                      className={`flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors ${
                        isActive
                          ? "bg-secondary text-foreground"
                          : "text-muted-foreground hover:bg-secondary hover:text-foreground"
                      }`}
                    >
                      <item.icon className="size-4" />
                      {item.label}
                    </Link>
                  </li>
                )
              })}
            </ul>
          </nav>
        </div>
        <div className="flex items-center gap-4">
          <ThemeToggle />
          <div className="hidden md:flex md:items-center md:gap-4">
            <Button variant="outline" size="icon" className="relative">
              <Bell className="size-4" />
              <Badge className="absolute -right-1 -top-1 size-2 bg-blue-500 p-0" />
            </Button>

            {connected ? (
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" className="flex items-center gap-2 px-2">
                    <Avatar className="size-8">
                      <AvatarImage src={userProfile?.avatar || "/placeholder.svg"} alt={userProfile?.fullName || "User"} />
                      <AvatarFallback className={`${avatarColor} text-white`}>
                        {userProfile?.fullName ? getInitials(userProfile.fullName) : "U"}
                      </AvatarFallback>
                    </Avatar>
                    <div className="flex flex-col items-start text-left">
                      <span className="text-sm font-medium">{userProfile?.fullName || "Guest User"}</span>
                      <span className="text-xs text-slate-400">{userProfile?.email || "guest@example.com"}</span>
                    </div>
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" className="w-56 bg-slate-900 border-slate-800">
                  <DropdownMenuLabel className="text-slate-400">My Account</DropdownMenuLabel>
                  <DropdownMenuSeparator className="bg-slate-800" />
                  <DropdownMenuItem className="text-slate-200 focus:bg-slate-800 focus:text-slate-200">
                    <User className="mr-2 size-4" />
                    Profile
                  </DropdownMenuItem>
                  <DropdownMenuItem className="text-slate-200 focus:bg-slate-800 focus:text-slate-200">
                    <Settings className="mr-2 size-4" />
                    Settings
                  </DropdownMenuItem>
                  <DropdownMenuSeparator className="bg-slate-800" />
                  <DropdownMenuItem 
                    className="text-slate-200 focus:bg-slate-800 focus:text-slate-200"
                    onClick={() => {
                      localStorage.removeItem('token')
                      setConnected(false)
                      window.location.href = '/'
                    }}
                  >
                    <LogOut className="mr-2 size-4" />
                    Logout
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            ) : (
              <Button
                onClick={() => setConnected(true)}
                className="bg-gradient-to-r from-blue-500 to-teal-400 text-white hover:opacity-90"
              >
                <Wallet className="mr-2 size-4" />
                Connect Wallet
              </Button>
            )}
          </div>

          <Sheet>
            <SheetTrigger asChild>
              <Button variant="outline" size="icon" className="md:hidden">
                <Menu className="size-4" />
                <span className="sr-only">Toggle menu</span>
              </Button>
            </SheetTrigger>
            <SheetContent side="right">
              <div className="flex flex-col gap-6 py-4">
                <Link href="/dashboard" className="flex items-center gap-2">
                  <div className="relative size-8 rounded-full bg-primary p-[1px]">
                    <div className="absolute inset-0 rounded-full bg-background p-1">
                      <div className="h-full w-full rounded-full bg-primary opacity-80"></div>
                    </div>
                  </div>
                  <span className="font-space text-xl font-bold tracking-tight text-white">UnityVault</span>
                </Link>

                <nav className="flex flex-col gap-2">
                  {navItems.map((item) => {
                    const isActive = pathname === item.href
                    return (
                      <Link
                        key={item.href}
                        href={item.href}
                        className={`flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors ${
                          isActive
                            ? "bg-secondary text-foreground"
                            : "text-muted-foreground hover:bg-secondary hover:text-foreground"
                        }`}
                      >
                        <item.icon className="size-4" />
                        {item.label}
                      </Link>
                    )
                  })}
                </nav>

                <div className="mt-auto">
                  {connected ? (
                    <div className="flex flex-col gap-4">
                      <div className="flex items-center gap-3">
                        <Avatar className="size-10">
                          <AvatarImage src={userProfile?.avatar || "/placeholder.svg"} alt={userProfile?.fullName || "User"} />
                          <AvatarFallback className={`${avatarColor} text-white`}>
                            {userProfile?.fullName ? getInitials(userProfile.fullName) : "U"}
                          </AvatarFallback>
                        </Avatar>
                        <div>
                          <div className="font-medium">{userProfile?.fullName || "Guest User"}</div>
                          <div className="text-xs text-slate-400">{userProfile?.email || "guest@example.com"}</div>
                        </div>
                      </div>
                      <Button
                        variant="outline"
                        className="w-full justify-start border-slate-700"
                        onClick={() => {
                          localStorage.removeItem('token')
                          setConnected(false)
                          window.location.href = '/'
                        }}
                      >
                        <LogOut className="mr-2 size-4" />
                        Logout
                      </Button>
                    </div>
                  ) : (
                    <Button
                      onClick={() => setConnected(true)}
                      className="w-full bg-gradient-to-r from-blue-500 to-teal-400 text-white hover:opacity-90"
                    >
                      <Wallet className="mr-2 size-4" />
                      Connect Wallet
                    </Button>
                  )}
                </div>
              </div>
            </SheetContent>
          </Sheet>
        </div>
      </div>
    </header>
  )
}
