import Link from "next/link";
import { Github, Globe, Twitter } from "lucide-react";

export default function Footer() {
    return (
        <footer className="py-6 md:py-8">
            <div className="flex justify-center items-center">
                <div className="flex space-x-4">
                    <Link
                        href="https://github.com/yorunoken"
                        className="text-muted-foreground hover:text-primary"
                        target="_blank"
                    >
                        <Github className="h-8 w-8" />
                        <span className="sr-only">GitHub</span>
                    </Link>
                    <Link
                        href="https://yorunoken.com"
                        className="text-muted-foreground hover:text-primary"
                        target="_blank"
                    >
                        <Globe className="h-8 w-8" />
                        <span className="sr-only">Personal Website</span>
                    </Link>
                    <Link
                        href="https://twitter.com/ken_yoru"
                        className="text-muted-foreground hover:text-primary"
                        target="_blank"
                    >
                        <Twitter className="h-8 w-8" />
                        <span className="sr-only">Twitter</span>
                    </Link>
                </div>
            </div>
        </footer>
    );
}
