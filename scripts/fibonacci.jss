push 16
save 0
push 0
print
push 1
start:
restore 0
push -1
add
save 0
add
print
save 2
restore 1
restore 2
save 1
restore 1
restore 0
ifeq 7
push 10101010
print
