import * as React from "react";
import { Modal, ModalHeader, ModalTitle, ModalDescription, ModalContent, ModalFooter } from "./ui/Modal";
import { Button } from "./ui/Button";
import { Github, Twitter, Mail, Heart, Youtube } from "lucide-react";

interface AboutModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const AboutModal: React.FC<AboutModalProps> = ({ isOpen, onClose }) => {
  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalHeader>
        <div className="flex items-center gap-3">
          <div className="flex items-center justify-center w-12 h-12 rounded-xl bg-gradient-to-br from-red-500 to-red-600 shadow-lg shadow-red-500/30">
            <Youtube className="w-7 h-7 text-white" />
          </div>
          <div>
            <ModalTitle>Lyricut YT Downloader</ModalTitle>
            <ModalDescription>Built with Tauri & React</ModalDescription>
          </div>
        </div>
      </ModalHeader>
      
      <ModalContent className="space-y-6">
        <div className="text-center">
          <p className="text-gray-300 text-sm leading-relaxed">
            A modern desktop application for downloading YouTube audio with automatic metadata tagging.
          </p>
        </div>
        
        <div className="flex flex-col items-center gap-4 py-4 border-y border-white/5">
          <div className="flex items-center gap-2 text-white">
            <span className="text-lg font-semibold">Made with</span>
            <Heart className="w-5 h-5 text-red-500 fill-red-500 animate-pulse-subtle" />
            <span className="text-lg font-semibold">by</span>
          </div>
          <div className="text-center">
            <h3 className="text-xl font-bold text-white">Ken Taylor</h3>
            <p className="text-gray-400 text-sm mt-1">Full Stack Developer</p>
          </div>
        </div>
        
        <div className="flex flex-col gap-3">
          <a
            href="https://github.com/ktappdev"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-3 p-3 rounded-lg bg-white/5 hover:bg-white/10 transition-colors group"
          >
            <Github className="w-5 h-5 text-gray-400 group-hover:text-white transition-colors" />
            <div className="flex-1">
              <p className="text-sm font-medium text-white">GitHub</p>
              <p className="text-xs text-gray-500">@ktappdev</p>
            </div>
            <span className="text-xs text-gray-500 group-hover:text-gray-300 transition-colors">
              github.com/ktappdev
            </span>
          </a>
          
          <a
            href="https://twitter.com/ktappdev"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-3 p-3 rounded-lg bg-white/5 hover:bg-white/10 transition-colors group"
          >
            <Twitter className="w-5 h-5 text-gray-400 group-hover:text-white transition-colors" />
            <div className="flex-1">
              <p className="text-sm font-medium text-white">Twitter</p>
              <p className="text-xs text-gray-500">@ktappdev</p>
            </div>
            <span className="text-xs text-gray-500 group-hover:text-gray-300 transition-colors">
              @ktappdev
            </span>
          </a>
          
          <a
            href="mailto:kentaylorappdev@gmail.com"
            className="flex items-center gap-3 p-3 rounded-lg bg-white/5 hover:bg-white/10 transition-colors group"
          >
            <Mail className="w-5 h-5 text-gray-400 group-hover:text-white transition-colors" />
            <div className="flex-1">
              <p className="text-sm font-medium text-white">Email</p>
              <p className="text-xs text-gray-500">Get in touch</p>
            </div>
            <span className="text-xs text-gray-500 group-hover:text-gray-300 transition-colors">
              kentaylorappdev@gmail.com
            </span>
          </a>
        </div>
      </ModalContent>
      
      <ModalFooter>
        <Button variant="outline" onClick={onClose}>
          Close
        </Button>
      </ModalFooter>
    </Modal>
  );
};

export { AboutModal };
