def try_ON(n, pont):
    print(pont)
    lens = to_length(n, pont)

    # print(lens)
    R_max = max(map(lambda x: x[1], filter(lambda x: x[0] == 0, lens)))
    R_min = min(map(lambda x: x[1], filter(lambda x: x[0] == 0, lens)))

    E_min = max(map(lambda x: x[1], filter(lambda x: x[0] == 1, lens)))

    counter = {i: 0 for i in range(R_min, R_max + 1)}
    for _, c in filter(lambda x: x[0] == 0, lens):
        counter[c] += 1

    # print(counter)

    zeon_pairs = []
    if lens[0][0] == 0:
        zeon_pairs.append((0, lens[0][1]))
    for idx, (c, i) in enumerate(lens):
        if c == 1:
            zeon_pairs.append((i, lens[idx + 1][1] if idx + 1 < len(lens) else 0))

    print(f"zeon_pairs: {zeon_pairs}")

    vals = []
    for i, (a, b) in enumerate(zeon_pairs):
        next = zeon_pairs[i + 1][0] if i + 1 < len(zeon_pairs) else 0
        vals.append((a + b + next, b))

    print(f"vals 1: {vals}")

    inter = []
    for o, _ in zeon_pairs:
        inter.append(o)

    print("inter :", inter)

    vals2 = merge(vals, inter)
    print(f"vals 2: {vals2}")

    vals_dict = {}
    vals_dict[0] = E_min

    for v, r in vals2:
        if r in vals_dict:
            vals_dict[r] = max(vals_dict[r], v)
        else:
            vals_dict[r] = max(v, E_min)

    print(vals_dict)

    vals_sorted = []
    E_max = 0
    for i in range(R_max + 1):
        if i in vals_dict:
            E_max = max(vals_dict[i], E_max)
            vals_sorted.append((E_max, i))

    print(vals_sorted)

    def forces():
        for i, (v, _) in enumerate(vals_sorted):
            next = vals_sorted[i + 1][1] if i + 1 < len(vals_sorted) else 0
            yield next - v

    print(list(forces()))

    force = max((r - E_min_with_R(n, lens, r) for r in range(R_min, R_max + 1)))

    print(force)

    return (force, force)

def try_NlogN(n, pont):
    lens = to_length(n, pont)
    Rs = get_Rs(n, lens)

    force = max((r - E_min_with_R(n, lens, r) for r in Rs))

    print(force)

def merge(vals, inter):
    vals_merged = []
    for i, (a, r) in enumerate(vals):
        if len(vals_merged) == 0:
            vals_merged.append((a, r))
        else:
            if vals_merged[-1][1] == r:
                vals_merged[-1] = (vals_merged[-1][0] + a - inter[i], r)
            else:
                vals_merged.append((a, r))

    print("merged :", vals_merged)

    acc_right = []
    for i, (a, r) in enumerate(vals_merged):
        if len(acc_right) == 0 or acc_right[-1][1] >= r:
            acc_right.append((a, r))
        elif acc_right[-1][1] < r:
            acc_right.append((a + acc_right[-1][0] - inter[i], r))

    print("acc_right :", acc_right)

    acc_left = []
    for i, (a, r) in reversed(list(enumerate(vals_merged))):
        if len(acc_left) == 0 or acc_left[-1][1] >= r:
            acc_left.append((a, r))
        elif acc_left[-1][1] < r:
            acc_left.append((a + acc_left[-1][0] - inter[i], r))

    acc_left.reverse()

    print("acc_left :", acc_left)

    combined = []
    for (ar, r), (al, _) in zip(acc_right, acc_left):
        combined.append((max(ar, al), r))

    print("combined :", combined)

    return combined


def test(size):
    tests = [[] for _ in range(2**size)]
    for i, l in enumerate(tests):
        for j in range(1, size + 1):
            if i % 2**j >= 2**(j-1):
                l.append(1)
            else:
                l.append(0)

    for t in tests:
        if 0 in t and 1 in t:
            res = try_ON(size, t)
            assert res[0] == res[1]

def get_Rs(n, lens):
    res = []
    for (c, l) in lens:
        if c == 0 and not l in res:
            res.append(l)
    return res



def E_min_with_R(n, lens, R):
    e = 0
    e_act = 0
    for cases in lens:
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
    try_NlogN(n, pont)
    # for i in range(2, 16):
    #     test(i)
    # try_NlogN(15, [0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0])
    # try_ON(15, [0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0])
    # try_NlogN(13, [0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0])
    # try_ON(13, [0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0])
