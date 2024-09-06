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
import { ArrowLeft, Upload, Info } from "lucide-react";
import { backend_url } from "@/lib";
import { useToast } from "@/hooks/use-toast";
import Link from "next/link";

export default function UploadPage() {
    const [file, setFile] = useState<File | null>(null);
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
        if (event.target.files && event.target.files[0]) {
            setFile(event.target.files[0]);
        }
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
        if (!file || !channelId) return;

        const reader = new FileReader();
        reader.onload = async (e) => {
            let threadId: number;
            let fileName: string;

            const fileData = new Uint8Array(e.target?.result as ArrayBuffer);

            setUploading(true);
            setProgress(0);

            const eventSource = new EventSource(
                `${backend_url}/api/upload/progress`,
            );

            const progressPromise = new Promise<void>((resolve, reject) => {
                eventSource.onmessage = async (event) => {
                    const progress = parseInt(event.data);
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
                        file: Array.from(fileData),
                        file_name: file.name,
                    }),
                });

                if (response.ok) {
                    const data = await response.json();
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
        };
        reader.readAsArrayBuffer(file);
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
                    Upload your file to your Discord server. <br />
                    The page might lag if the file you are uploading exceeds
                    200MiB, please be patient.
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
                            <InfoBox />
                        </div>
                        {channelError && (
                            <span className="text-red-700 text-sm">
                                {channelError}
                            </span>
                        )}
                        <Input
                            className="mt-2"
                            type="file"
                            onChange={handleFileChange}
                            disabled={uploading}
                        />
                    </div>
                    {uploading && (
                        <div className="space-y-2">
                            <Progress value={progress} className="w-full" />
                            <p className="text-sm text-gray-500 text-center">
                                {progress}% Uploaded
                            </p>
                        </div>
                    )}
                </div>
            </CardContent>
            <CardFooter>
                <Button
                    className="w-full"
                    onClick={uploadFile}
                    disabled={!file || !channelId || uploading}
                >
                    {uploading ? "Uploading..." : "Start Upload"}
                    {!uploading && <Upload className="ml-2 h-4 w-4" />}
                </Button>
            </CardFooter>
        </Card>
    );
}

function InfoBox() {
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
