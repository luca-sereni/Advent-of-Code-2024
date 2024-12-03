file_to_read = open("day2/input.txt", "r")

reports = []

for line in file_to_read:
    levels = []
    for element in line.split():
        levels.append(int(element))
    reports.append(levels)

file_to_read.close()

""" FIRST PART
safe_reports = 0
is_safe = True
option = None

for report in reports:
    is_safe = True
    option = None
    for i in range(len(report) - 1):
        if abs(report[i] - report[i + 1]) < 1 or abs(report[i] - report[i + 1]) > 3:
            is_safe = False
            break

        if report[i] < report[i + 1]:
            if option == "decreasing":
                is_safe = False
                break
            elif option == None:
                option = "increasing"
        else:
            if option == "increasing":
                is_safe = False
                break
            elif option == None:
                option = "decreasing"
    if is_safe == True:
        safe_reports += 1

print(safe_reports)
"""

def check_report(temp_report) -> bool:
    is_safe = True
    option = None
    for i in range(len(temp_report) - 1):
        if abs(temp_report[i] - temp_report[i + 1]) < 1 or abs(temp_report[i] - temp_report[i + 1]) > 3:
            is_safe = False
            break

        if temp_report[i] < temp_report[i + 1]:
            if option == "decreasing":
                is_safe = False
                break
            elif option == None:
                option = "increasing"
        else:
            if option == "increasing":
                is_safe = False
                break
            elif option == None:
                option = "decreasing"
    return is_safe

safe_reports = 0
for report in reports:
    is_safe = True
    option = None
    for i in range(len(report) - 1):
        if abs(report[i] - report[i + 1]) < 1 or abs(report[i] - report[i + 1]) > 3:
            print(report)
            temp_report1 = report[:i] + report[i + 1:]
            print(temp_report1)
            is_safe1 = check_report(temp_report1)

            temp_report2 = report[:i + 1] + report[i + 2:]
            print(temp_report2)
            is_safe2 = check_report(temp_report2)
            is_safe = is_safe1 or is_safe2
            break

        if report[i] < report[i + 1]:
            if option == "decreasing" and i == 1:
                temp_report0 = report[i:]
                temp_report1 = report[:i] + report[i + 1:]
                temp_report2 = report[:i + 1] + report[i + 2:]
                is_safe0 = check_report(temp_report0)
                is_safe1 = check_report(temp_report1)

                is_safe2 = check_report(temp_report2)
                is_safe = is_safe0 or is_safe1 or is_safe2
                break
            elif option == "decreasing" and i != 1:
                temp_report1 = report[:i] + report[i + 1:]
                temp_report2 = report[:i + 1] + report[i + 2:]
                is_safe1 = check_report(temp_report1)

                is_safe2 = check_report(temp_report2)
                is_safe = is_safe1 or is_safe2
                break
            elif option == None:
                option = "increasing"
        else:
            if option == "increasing" and  i == 1: #check the first two position (determine the correct trend -> ascending, descending)
                temp_report0 = report[i:]
                temp_report1 = report[:i] + report[i + 1:] 
                temp_report2 = report[:i + 1] + report[i + 2:] 
                is_safe0 = check_report(temp_report0)
                is_safe1 = check_report(temp_report1)

                is_safe2 = check_report(temp_report2)
                is_safe = is_safe0 or is_safe1 or is_safe2
                break
            elif option == "increasing" and i != 1:
                temp_report1 = report[:i] + report[i + 1:]
                temp_report2 = report[:i + 1] + report[i + 2:]
                is_safe1 = check_report(temp_report1)

                is_safe2 = check_report(temp_report2)
                is_safe = is_safe1 or is_safe2
                break
            elif option == None:
                option = "decreasing"
    if is_safe == True:
        safe_reports += 1

print(safe_reports)