# Undercover 🕵️‍♂️

**Undercover** to towarzyska gra dedukcyjna, w której jeden z graczy zostaje kłamcą i musi wtopić się w grupę, nie znając tajnego hasła. Gra jest przeznaczona do rozgrywki w gronie znajomych i opiera się na skojarzeniach, obserwacji oraz blefie.

> [!IMPORTANT]  
> This is a simple party game designed for Polish-speaking players.  
> The gameplay and UI are in Polish, but the source code is written in English.

## 🎯 Cel gry

- **Gracze**: odkryć, kto jest kłamcą
- **Undercover (kłamca)**: nie zostać zdemaskowanym

## 🧠 Zasady gry

1. **Przygotowanie gry**
   - Jedna osoba zostaje losowo wybrana jako **Undercover**
   - Undercover **nie zna hasła**
   - Zamiast tego otrzymuje **krótką podpowiedź**
   - Pozostali gracze **znają tajne hasło**

2. **Rundy**
   - Gra toczy się w rundach
   - W każdej rundzie każdy gracz po kolei podaje **jedno skojarzenie** związane z hasłem (skojarzenie max 2 słowa)
   - Undercover musi improwizować i udawać, że zna hasło

3. **Głosowanie**
   - Po ustalonej liczbie rund:
     - Domyślnie **4 rundy**
     - Liczbę rund można ustalić przed rozpoczęciem gry
   - Gracze **głosują na osobę**, która ich zdaniem nie znała hasła

4. **Warunki zwycięstwa**
   - **Gracze wygrywają**, jeśli poprawnie wskażą Undercover'a
   - **Undercover wygrywa**, jeśli nie zostanie odkryty

5. **Inny sposób wygranej**
   - Jeśli Undercover uzna że zna hasło to może powiedzieć: "Strzelam, HASŁO"
   - Jeśli to było poprawne hasło wygrywa, jeśli nie przegrywa

## 🛠️ Technologie

- **Framework**: [Dioxus](https://dioxuslabs.com/)
- **Język**: Rust
- **Platformy**: Android

## 🚀 Status projektu

Projekt jest w trakcie tworzenia.

## 📌 Informacje dodatkowe

- Najlepiej grać w gronie znajomych, siedząc razem. Zalecana nie pażysta ilość graczy
- Gra nie wymaga połączenia z internetem
- Kluczowe elementy rozgrywki to blef, kreatywność i dedukcja
