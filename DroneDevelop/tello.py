from tkinter import *
import tkinter.messagebox as messagebox

width = 1280
height = 720

window = Tk()

window.minsize(width, height)
# to rename the title of the window
window.title("Tello App")
# pack is used to show the object in the window
messagebox.showinfo("Reminder", "Please connect to Tello wifi")
container = Frame()
scrollbar = Scrollbar(container)
scrollbar.pack(side="right", fill="y")
Label(container, text = "Yay").pack(side="left", fill="both")
window.mainloop()