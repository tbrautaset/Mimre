import math, random, sys, pathlib, pygame
root = pathlib.Path(__file__).resolve().parent.parent
assets = root / "assets"
pygame.init(); pygame.mixer.init()
pygame.mixer.music.load(assets / "music.wav"); pygame.mixer.music.play(-1)
W, H = 640, 520
screen = pygame.display.set_mode((W, H))
font = pygame.font.Font(r"C:\Windows\Fonts\arial.ttf", 24)
clock = pygame.time.Clock()
top = "        Dette er en liten test laget med Python  – et lite tilbakeblikk på ungdomstiden og gamle spillminner. Tenk Tiki-100 og Commodore 64! En hyllest til den tida da alt handlet om kreativitet, nysgjerrighet og pixler i bevegelse.        "
bot = "          Jeg husker NinjaWriter som en avansert teksteditor på Commodore 64 – perfekt for å lage imponerende tekst. Vi brukte den til å skrive meldinger til hverandre i hverdagen, og diskettene ble nesten glødende i posten. Man fant på mye rart den gangen! Det var spennende å eksperimentere med sprites, inspirert av Commodore 64 Programmer's Reference Guide, den gamle programmeringsboka vi alltid hadde for hånden. Enten det var i BASIC eller 6502 assembler, handlet det om å prøve, feile – og lære.        "
sprite = pygame.image.load(assets / "sprite.bmp").convert(); sprite.set_colorkey(sprite.get_at((0, 0)))
stars = [(random.randrange(W), random.randrange(H), (random.randrange(256), random.randrange(256), random.randrange(256))) for _ in range(100)]
bars = [(255,255,85),(255,64,64),(85,85,255),(139,69,19)]
def grad(c): return [tuple(min(cc+i,255) for cc in c) for i in [0,40,80,120,160,200,255,200,160,120,80,40,0]]
grads = [grad(c) for c in bars]
r, cx, cy, angle = 100.0, 320.0, 240.0, 0.0
bar_h, top_y, bot_y = H//4, 0, 460
start = pygame.time.get_ticks()
def draw_bars(t):
    for i,g in enumerate(grads):
        off = math.sin(t*2+i*0.5)*40; boff=-off
        for k in range(13):
            col=g[k]; y=k+off
            if 0<=y<bar_h: screen.fill(col, pygame.Rect(0,int(y+top_y),W,1))
            yb=k+boff+bot_y
            if bot_y<=yb<bot_y+bar_h: screen.fill(col, pygame.Rect(0,int(yb),W,1))
def scroll(txt,off,y):
    surf=font.render(txt,True,(255,255,255)); w=surf.get_width(); x=int(off)%w-w
    screen.blit(surf,(x,y)); screen.blit(surf,(x+w,y))
while True:
    for e in pygame.event.get():
        if e.type==pygame.QUIT: pygame.quit(); sys.exit()
    t=(pygame.time.get_ticks()-start)/1000
    screen.fill((0,0,0))
    draw_bars(t); scroll(top,-t*100,10); scroll(bot,t*100,bot_y)
    for x,y,c in stars: screen.fill(c,pygame.Rect(x,y,2,2))
    sx=cx+r*math.cos(angle); sy=cy+r*math.sin(angle); screen.blit(sprite,(sx,sy)); angle+=0.02
    pygame.display.flip(); clock.tick(60)
