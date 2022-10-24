import pygame, csv, sys

pygame.init()
clock = pygame.time.Clock()
window = pygame.display.set_mode((500, 500))


if __name__=='__main__':
    while True:
        with open('./output.csv') as f:
            data = f.read().strip()
            ndarray = data.split("\n")
            for row in ndarray:
                ndarray[ndarray.index(row)] = [float(i) for i in row.split(",")]
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                sys.exit()
        for y in range(500):
            for x in range(500):
                level = ndarray[y][x]
                color = (level, level, level)


                if level < 10:
                    color = (10, 30, 78)                
                elif level < 15:
                    color = (10, 50, 100) 
                elif level < 30:
                    color = (10, 70, 150) 
                elif level < 40:
                    color = (10, 100, 200)                
                elif level < 50:
                    color = (50, 150, 250)
                elif level < 55:
                    color = (250, 250, 170)
                elif level < 60:
                    color = (150, 150, 110)
                elif level < 75:
                    color = (100, 200, 100)
                elif level < 100:
                    color = (70, 170, 70)                
                elif level < 130:
                    color = (50, 150, 50)                
                elif level < 145:
                    color = (50, 120, 50)                
                elif level < 160:
                    color = (70, 60, 50)
                elif level < 180:
                    color = (60, 50, 40)
                elif level < 195:
                    color = (50, 40, 30)
                elif level < 220:
                    color = (220, 220, 255)
                else:
                    color = (255, 255, 255)
                window.set_at((x, y), color)
        pygame.display.flip()
        clock.tick(24)