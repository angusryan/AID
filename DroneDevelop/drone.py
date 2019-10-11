from socket import socket, AF_INET, SOCK_DGRAM
import os.path
from os import path, system
from time import sleep

#GLOBAL VARIABLES#
droneAddress = "192.168.10.1"
dronePort = 8889


#FUNCTIONS#
#Used in multiple other functions
#setup function
def setup():
    serverSocket = socket(AF_INET, SOCK_DGRAM)
    serverSocket.bind(('', 8889))
    msg = "command"
    serverSocket.sendto(msg.encode(), (droneAddress, dronePort))
    print(serverSocket.recv(1024))
    return serverSocket

#To be implemented in the future
#create a new mission
def newMission():
    print("Feature to be added in a later version")

#edit an existing mission
def editMission():
    print("Feature to be added in a later version")

#delete an existing mission
def deleteMission():
    print("Feature to be added in a later version")

#Incomplete...
#run an existing mission
def runMission(file, serverSocket):
    for line in file:
        print(line[0:4])
        if line[0:4] == "wait":
            time = int(line[5:])
            sleep(time)
        else:
            msg = line[0:-1]
            print(msg)
            serverSocket.sendto(msg.encode(), (droneAddress, dronePort))
            print(serverSocket.recv(1024))

#command handling:
#command: connect
def connect():
    serverSocket = setup()
    exit = False
    while not exit:
        msg = input("Tello command: ")
        if msg == "exit":
            exit = True
        else:
            serverSocket.sendto(msg.encode(), (droneAddress, dronePort))
            print(serverSocket.recv(1024))

#command: run
def run():
    userInput = input("Mission: ")
    filename = userInput + ".txt"
    if filename == "help.txt":
        filename = "missions.txt"
        file = open(filename, 'r')
        file.close

    elif path.exists(filename):
        serverSocket = setup()
        file = open(filename, 'r')
        runMission(file, serverSocket)
        file.close

    else:
        print("Error 404: File not found!")

#command: new
def new():
    userInput = input("Do you wish to create a new mission? (y/n)>")
    if userInput == "y":
        newMission()
    else:
        print("Cancelled")

#command: edit
def edit():
    userInput = input("Do you wish to create edit an existing mission? (y/n)>")
    if userInput == "y":
        editMission()
    else:
        print("Cancelled")

#command: delete
def delete():
    userInput = input("Do you wish to create delete an existing mission? (y/n)>")
    if userInput == "y":
        deleteMission()
    else:
        print("Cancelled")

#command: help
def help():
    file = open("help.txt")
    for line in file:
        print(line)
    file.close

#main loop
exit = False
while not exit:
    userInput = input("Command: ")

    if userInput == "exit":
        exit = True
    elif userInput == "connect":
        connect()
    elif userInput == "run":
        run()
    elif userInput == "clear":
        os.system('cls' if os.name == 'nt' else 'clear')
    elif userInput == "new":
        new()
    elif userInput == "edit":
        edit()
    elif userInput == "delete":
        delete()
    elif userInput == "help" or userInput == "h":
        help()
    else:
        print("Invalid command: Type help for list of commands")