"use client";

import { Button } from "@/components/ui/button";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import Link from "next/link";

export default function MainPage() {
    return (
        <Card className="w-full max-w-md p-6">
            <CardHeader className="text-center">
                <CardTitle>Discord Mass Uploader</CardTitle>
                <CardDescription>
                    Easily upload your files to your Discord server! <br />
                    To get started, select what operation you want to do.
                </CardDescription>
            </CardHeader>
            <CardContent>
                <div className="flex justify-center space-x-4">
                    <Link href="/upload">
                        <Button>Upload</Button>
                    </Link>
                    <Link href="/download">
                        <Button>Download</Button>
                    </Link>
                </div>
            </CardContent>
        </Card>
    );
}
