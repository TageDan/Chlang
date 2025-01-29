import random
import base64

import os

class Bot:
    def __init__(self, string):
        self.byte_list = base64.b64decode(string)
        self.score = 0

    def reset_mutate(self):
        new = Bot(base64.b64encode(self.byte_list))
        for b in new.byte_list:
            if random.randint(0, 20) > 19:
                b += random.randint(-5,5)
                b = min(max(b, 0), 255)
        return new

    def b64(self):
        return base64.b64encode(self.byte_list).decode("ascii")


def random_bot():
    return Bot(base64.b64encode(random.randbytes(390)))

def run_game(bot1,bot2):
    return int(os.popen(f"target/release/chlang POSITIONAL 3 {base64.b64encode(bot1.byte_list).decode("ascii")} POSITIONAL 3 {base64.b64encode(bot2.byte_list).decode("ascii")}").read())


os.popen("cargo build --release --features compare").close() 
# print("hej")

with open("training.log", "a+") as f:
    l = f.readlines()                
    if len(l) < 3:
        bots = []
        for line in l:
            bots.append(Bot(line))
    else:
        bots = []
        for line in l[-3:]:
            bots.append(Bot(line))
    while len(bots) < 6:
        bots.append(random_bot())
    while True:

        for b1 in range(0, len(bots)):
            for b2 in range(0,len(bots)):
                if b1 == b2:
                    continue;
                res = run_game(bots[b1],bots[b2])
                bots[b1].score += res
                bots[b2].score -= res

        
        bots.sort(key = lambda x: x.score, reverse=True)
        n_bots = bots[:2]
        n_bots[0].score = 0
        n_bots[1].score = 0
        n_bots.append(bots[0].reset_mutate())
        n_bots.append(  bots[0].reset_mutate())
        n_bots.append(  bots[1].reset_mutate())
        n_bots.append(  random_bot())
        bots = n_bots
        print(bots[0].b64())
        f.writelines([bots[0].b64()+"\n", bots[1].b64()+"\n", bots[2].b64()+"\n"])
        
                
                    


