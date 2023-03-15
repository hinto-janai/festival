library(ggplot2)

# Festival.
x.festival <- c(0)
y.festival <- c(0, 5)

# Lollypop.
x.lollypop <- c(0)
y.lollypop <- c(0, 60)

# MusicBee.
x.musicbee <- c(0)
y.musicbee <- c(0, 24)

# GNOME Music.
x.gnome <- c(0)
y.gnome <- c(0, 32)

# iTunes.
x.itunes <- c(0)
y.itunes <- c(0, 97)

# Data.
input.data <- rbind(
  data.frame(x = x.festival, y = y.festival, series = "Festival (5s)"),
  data.frame(x = x.musicbee, y = y.musicbee, series = "MusicBee (24s)"),
  data.frame(x = x.gnome,    y = y.gnome,    series = "GNOME Music (32s)"),
  data.frame(x = x.lollypop, y = y.lollypop, series = "Lollypop (60s)"),
  data.frame(x = x.itunes,   y = y.itunes,   series = "iTunes (97s)")
)

# Bar plot.
p <- ggplot(aes(x = x, y = y, fill = series), data = input.data) +
	geom_col(position = "dodge", width = 15) +
	scale_x_discrete(labels = NULL) +
	scale_y_continuous(breaks = scales::pretty_breaks(n = 25)) +
	theme(text = element_text(size = 30)) +
	theme(plot.title = element_text(color = "black", size = 40, face = "bold")) +
	labs(title = "New Collection (from scratch)", y = "Seconds (less is better)", x = NULL, fill = NULL)

# Create PNG.
png("scratch.png", width = 1000, height = 1000, pointsize = 50, res = 75)
print(p)
dev.off()
