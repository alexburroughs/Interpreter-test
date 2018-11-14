push 16
save 0
push 0
print
save 1
push 1
print
save 2
start:
restore 1
restore 2
save 1
restore 1
add
print
save 2
restore 0
ifeq 24
restore 0
push -1
add
save 0
jumpa start
push 10101010
print
