def f_max(n, pont):
    lens = to_length(n, pont)

    # print(lens)
    R_max = max(map(lambda x: x[1], filter(lambda x: x[0] == 0, lens)))

    force = max((r - E_min_with_R(n, lens, r) for r in range(1, R_max + 1)))

    print(force)


def E_min_with_R(n, lens, R):
    e = 0
    e_act = 0
    for cases in lens:

        # Penser à faire la comparaison à la fin
        if cases[0] == 0 and cases[1] >= R:
            # print(e, e_act)
            e = e if e >= e_act else e_act
            e_act = 0
            # print(f"Walking {e}")
        else:
            # print("Swiming")
            e_act += cases[1]
            # print(e_act)

    e = e_act if e_act >= e else e
    # print(f'Inside E_min_with_R. R = {R}, e = {e}')
    return e
    
        
def to_length(n, pont):
    res = []
    last = pont[0]
    counter = 0
    for c in pont:
        if c != last:
            res.append((last, counter))
            counter = 1
            last = c
        else:
            counter += 1
    res.append((last, counter))
    return res


if __name__ == "__main__":
    n = int(input())
    pont = list(map(int, input().split()))
    f_max(n, pont)
