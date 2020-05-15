range_lower = 235741
range_upper = 706948


def is_password_valid(password):
    number_vector = str(password)
    adjacent = False
    if number_vector[0] == number_vector[1] or number_vector[1] == number_vector[2] or number_vector[2] == number_vector[3] or number_vector[3] == number_vector[4] or number_vector[4] == number_vector[5]:
        adjacent = True

    if not adjacent:
        return False
    my_keys = {}
    for i in number_vector:
        if my_keys.get(i):
            my_keys[i] += 1
        else:
            my_keys[i] = 1
    is_double = False
    for i in my_keys:
        if(my_keys.get(i) == 2):
            is_double = True
            break
    if not is_double:
        return False
    if number_vector[0] <= number_vector[1] and number_vector[1] <= number_vector[2] and number_vector[2] <= number_vector[3] and number_vector[3] <= number_vector[4] and number_vector[4] <= number_vector[5]:
        return True
    return False


counter = 0
assert is_password_valid(112233) == True
assert is_password_valid(123444) == False
assert is_password_valid(111122) == True


for i in range(range_lower, range_upper+1):
    if is_password_valid(i):
        counter += 1
print("Total password is ", counter)
