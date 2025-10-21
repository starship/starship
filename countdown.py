#!/usr/bin/env python3
"""
Compteur décrémental partant de 10
Countdown counter starting from 10
"""

import time


def countdown(start=10):
    """
    Compte à rebours depuis une valeur de départ jusqu'à 0
    Countdown from a starting value to 0

    Args:
        start (int): Valeur de départ du compteur (default: 10)
    """
    for i in range(start, -1, -1):
        print(f"Compteur: {i}")
        time.sleep(1)

    print("Terminé!")


if __name__ == "__main__":
    print("Démarrage du compteur décrémental...")
    countdown(10)
