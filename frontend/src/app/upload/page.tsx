"use client";

import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Progress } from "@/components/ui/progress";
import Cookies from "js-cookie";
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog";
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
import { ArrowLeft, Upload, Info } from "lucide-react";
import { backend_url } from "@/lib";
import { useToast } from "@/hooks/use-toast";
import Link from "next/link";

export default function UploadPage() {
    const [filePath, setFilePath] = useState<string | null>(null);
    const [channelId, setChannelId] = useState<string | null>(null);
    const [uploading, setUploading] = useState(false);
    const [progress, setProgress] = useState(0);
    const [channelError, setChannelError] = useState<string | null>(null);

    const { toast } = useToast();

    useEffect(() => {
        const cookieId = Cookies.get("discordChannelId");
        if (cookieId) {
            setChannelId(cookieId);
        }
    }, []);

    function handleFileChange(event: React.ChangeEvent<HTMLInputElement>) {
        const { value } = event.target;

        setFilePath(value);
    }

    function handleDiscordChannelChange(
        event: React.ChangeEvent<HTMLInputElement>,
    ) {
        const { value } = event.target;

        if (value === "") {
            setChannelError(null);
            Cookies.remove("discordChannelId");
        } else if (!isNaN(Number(value))) {
            Cookies.set("discordChannelId", value, { expires: 30 });
            setChannelError(null);
        } else {
            setChannelError("Value isn't a number.");
        }

        setChannelId(event.target.value);
    }

    async function uploadFile() {
        if (!filePath || !channelId) return;

        let threadId: number;
        let fileName: string;

        setUploading(true);
        setProgress(0);

        const eventSource = new EventSource(
            `${backend_url}/api/upload/progress`,
        );

        const progressPromise = new Promise<void>((resolve, reject) => {
            eventSource.onmessage = async (event) => {
                const progress = Number(event.data);
                setProgress(progress);
                if (progress === 100) {
                    eventSource.close();

                    console.log("finished uploading");
                    const response = await fetch(
                        `${backend_url}/api/database/file`,
                        {
                            method: "POST",
                            headers: {
                                "Content-Type": "application/json",
                            },
                            body: JSON.stringify({
                                thread_id: threadId,
                                file_name: fileName,
                            }),
                        },
                    );
                    console.log(response);

                    resolve();
                }
            };

            eventSource.onerror = (error) => {
                console.error("SSE Error:", error);
                eventSource.close();
                setUploading(false);
                reject(new Error("Progress tracking failed"));
            };
        });

        try {
            const response = await fetch(`${backend_url}/api/upload`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    channel_id: channelId,
                    file_path: filePath,
                }),
            });

            if (response.ok) {
                const data = await response.json();
                console.log(data);
                threadId = data.thread_id;
                fileName = data.file_name;
            } else {
                toast({
                    title: "Uh oh! Something went wrong.",
                    description:
                        "There was an error while uploading your file, please check the logs.",
                });
            }

            await progressPromise;

            toast({
                title: "Success!",
                description: "Your file has been uploaded successfully.",
            });
        } catch (error) {
            console.error(error);
            setUploading(false);
            toast({
                title: "Uh oh! Something went wrong.",
                description:
                    "There was a network error, please check the logs.",
            });
        } finally {
            setUploading(false);
        }
    }

    return (
        <Card className="w-full max-w-md p-6 relative">
            <Link className="absolute top-1 left-1" href="/">
                <Button variant="ghost">
                    <ArrowLeft className="mr-2 h-4 w-4" />
                    Back
                </Button>
            </Link>
            <CardHeader className="text-center">
                <CardTitle>Upload Module</CardTitle>
                <CardDescription>
                    Upload your file to your Discord server.
                </CardDescription>
            </CardHeader>
            <CardContent>
                <div className="space-y-4">
                    <div className="flex flex-col items-center">
                        <div className="relative flex items-center w-full">
                            <Input
                                type="text"
                                placeholder="Discord channel ID here."
                                value={channelId ?? ""}
                                onChange={handleDiscordChannelChange}
                                disabled={uploading}
                                className="flex-grow"
                            />
                            <InfoBoxDiscordChannel />
                        </div>
                        {channelError && (
                            <span className="text-red-700 text-sm">
                                {channelError}
                            </span>
                        )}
                        <div className="relative flex items-center w-full mt-2">
                            <Input
                                type="text"
                                placeholder="File path here..."
                                value={filePath ?? ""}
                                onChange={handleFileChange}
                                disabled={uploading}
                            />
                            <InfoBoxFilePath />
                        </div>
                        <span className="text-xs text-muted-foreground">
                            We need the file path here otherwise it gets very
                            laggy the larger the file is.
                        </span>
                    </div>
                    {uploading && (
                        <div className="space-y-2">
                            <Progress value={progress} className="w-full" />
                            <p className="text-sm text-gray-500 text-center">
                                {progress.toFixed(2)}% Uploaded
                            </p>
                        </div>
                    )}
                </div>
            </CardContent>
            <CardFooter>
                <Button
                    className="w-full"
                    onClick={uploadFile}
                    disabled={!filePath || !channelId || uploading}
                >
                    {uploading ? "Uploading..." : "Start Upload"}
                    {!uploading && <Upload className="ml-2 h-4 w-4" />}
                </Button>
            </CardFooter>
        </Card>
    );
}

function InfoBoxDiscordChannel() {
    return (
        <Dialog>
            <DialogTrigger asChild>
                <Info className="absolute right-2 w-5 cursor-pointer text-gray-400 hover:text-gray-200 transition-colors" />
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>How to get a Discord channel ID</DialogTitle>
                </DialogHeader>
                <DialogDescription>
                    <ol className="list-decimal list-inside space-y-2">
                        <li>
                            Enable Developer Mode in Discord (User Settings{" "}
                            {">"} App Settings {">"} Advanced {">"} Developer
                            Mode)
                        </li>
                        <li>
                            Right-click on the desired channel in your Discord
                            server
                        </li>
                        <li>
                            Click on {"'"}Copy ID{"'"} at the bottom of the
                            context menu
                        </li>
                        <li>Paste the copied ID into the input field</li>
                    </ol>
                </DialogDescription>
            </DialogContent>
        </Dialog>
    );
}

function InfoBoxFilePath() {
    return (
        <Dialog>
            <DialogTrigger asChild>
                <Info className="absolute right-2 w-5 cursor-pointer text-gray-400 hover:text-gray-200 transition-colors" />
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>How to get file path</DialogTitle>
                </DialogHeader>
                <DialogDescription>
                    <p className="mb-2">
                        Select your operating system for instructions:
                    </p>
                    <Accordion type="single" collapsible className="w-full">
                        <AccordionItem value="windows">
                            <AccordionTrigger>Windows</AccordionTrigger>
                            <AccordionContent>
                                <ol className="list-decimal list-inside space-y-1 ml-2">
                                    <li>
                                        Open File Explorer and navigate to your
                                        file
                                    </li>
                                    <li>
                                        Right-click on the file and select
                                        {'"'}Properties{'"'}
                                    </li>
                                    <li>
                                        In the Properties window, find the
                                        {'"'}Location{'"'} field
                                    </li>
                                    <li>
                                        Copy the entire path and add the
                                        filename at the end
                                    </li>
                                </ol>
                            </AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="macos">
                            <AccordionTrigger>macOS</AccordionTrigger>
                            <AccordionContent>
                                <ol className="list-decimal list-inside space-y-1 ml-2">
                                    <li>Open Finder and locate your file</li>
                                    <li>
                                        Right-click (or Control-click) on the
                                        file
                                    </li>
                                    <li>
                                        Hold the Option key and select {'"'}Copy
                                        [filename] as Pathname{'"'}
                                    </li>
                                    <li>Paste this into the input field</li>
                                </ol>
                            </AccordionContent>
                        </AccordionItem>
                        <AccordionItem value="linux">
                            <AccordionTrigger>Linux</AccordionTrigger>
                            <AccordionContent>
                                <ol className="list-decimal list-inside space-y-1 ml-2">
                                    <li>
                                        Open your file manager and navigate to
                                        the file
                                    </li>
                                    <li>
                                        Right-click on the file and select
                                        {'"'}Properties{'"'}
                                    </li>
                                    <li>
                                        Look for the {'"'}Location{'"'} or {'"'}
                                        Path{'"'} field
                                    </li>
                                    <li>
                                        Copy the full path and add the filename
                                        at the end
                                    </li>
                                </ol>
                            </AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </DialogDescription>
            </DialogContent>
        </Dialog>
    );
}
