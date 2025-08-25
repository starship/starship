;;; starship.el --- Description -*- lexical-binding: t; -*-
;;
;; Copyright (C) 2025 Starship Contributors
;; This file is not part of GNU Emacs.
;;
;;; Commentary:
;;
;;  This is meant to be evaluated in someone's config,
;;  not be a proper bona fide elisp library
;;
;;  Description
;;
;; Starship init for eshell
;;
;;; Code:

(defvar starship/eshell-is-first-exec t
  "implementation detail for internal starship use")
(defvar starship/eshell-last-command-time nil
  "the time that the last eshell command started")
(defun starship/reset-eshell-is-first-exec ()
  (setq starship/eshell-is-first-exec t))
(defun starship/set-eshell-last-command-time (_cmd)
  (when starship/eshell-is-first-exec
    (setq starship/eshell-last-command-time (current-time))
    (setq starship/eshell-is-first-exec nil)))
(defun starship/eshell-starship-prompt ()
  (with-temp-buffer
    (with-environment-variables (("TERM" "eshell-is-not-dumb")
                                 ("STARSHIP_SHELL" "eshell")
                                 ("STARSHIP_SESSION_KEY"
                                  (format "%016d" (random (expt 10 16)))))
      (call-process "::STARSHIP::"
                    nil t nil "prompt"
                    (concat
                     "--status=" (number-to-string eshell-last-command-status))
                    (concat
                     "--jobs="
                     (number-to-string (length eshell-background-commands)))
                    (concat
                     "--cmd-duration="
                     (number-to-string
                      (if (and starship/eshell-last-command-time
                               (not starship/eshell-is-first-exec))
                          (car
                           (time-convert (time-subtract
                                          (current-time)
                                          starship/eshell-last-command-time)
                                         1000))
                        0)))))
    (require 'ansi-color)
    (let ((ansi-color-apply-face-function
           #'ansi-color-apply-text-property-face))
      (ansi-color-apply-on-region (point-min) (point-max)))
    ;; i have no idea why this hack is required...
    ;; let's just go with it for now
    (let ((the-string (buffer-string)))
      (erase-buffer)
      (goto-char (point-min))
      (insert-for-yank the-string))
    (buffer-string)))

(add-hook 'eshell-after-prompt-hook #'starship/reset-eshell-is-first-exec)
(add-hook 'eshell-exec-hook #'starship/set-eshell-last-command-time)
(setq eshell-prompt-function #'starship/eshell-starship-prompt)

;;; starship.el ends here
