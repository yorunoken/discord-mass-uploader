"use client";

import { DeleteConfirmationDialog } from "@/components/deleteConfirmation";
import { Button } from "@/components/ui/button";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { Progress } from "@/components/ui/progress";
import { useToast } from "@/hooks/use-toast";
import { backend_url } from "@/lib";
import { ArrowLeft, Download } from "lucide-react";
import Link from "next/link";
import { useEffect, useState } from "react";

type Files = {
    file_name: string;
    thread_id: string;
};

export default function DownloadPage() {
    const [files, setFiles] = useState<Array<Files> | null>(null);
    const [progress, setProgress] = useState<Record<string, number>>({});
    const [activeDownloads, setActiveDownloads] = useState<
        Record<string, boolean>
    >({});
    const { toast } = useToast();

    async function downloadFile(fileName: string, threadId: string) {
        toast({
            title: `Downloading`,
            description: `Downloading ${fileName} from thread ${threadId}`,
        });

        setProgress((prev) => ({ ...prev, [fileName]: 1 }));
        setActiveDownloads((prev) => ({ ...prev, [fileName]: true }));

        const eventSource = new EventSource(
            `${backend_url}/api/download?thread_id=${threadId}&file=${fileName}`,
        );

        eventSource.onmessage = (event) => {
            const currentProgress = Number(event.data);
            console.log("Download progress:", currentProgress);

            setProgress((prev) => ({
                ...prev,
                [fileName]: currentProgress,
            }));

            if (currentProgress === 100) {
                eventSource.close();
                setActiveDownloads((prev) => ({
                    ...prev,
                    [fileName]: false,
                }));

                toast({
                    title: `Download finished!`,
                    description: `Finished the download of ${fileName}.`,
                });
            }
        };

        eventSource.onerror = (error) => {
            console.error("Download error:", error);
            eventSource.close();

            setProgress((prev) => ({
                ...prev,
                [fileName]: 0,
            }));
            setActiveDownloads((prev) => ({ ...prev, [fileName]: false }));

            toast({
                title: `Uh oh!`,
                description: `Something went wrong while downloading ${fileName} from thread ${threadId}`,
            });
        };
    }

    async function deleteFileFromDb(fileName: string, threadId: string) {
        await fetch(`${backend_url}/api/database/file/delete`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                thread_id: threadId,
                file_name: fileName,
            }),
        });

        const response = await fetch(`${backend_url}/api/files`);

        if (!response.ok) return;
        const data: Array<Files> = await response.json();
        setFiles(data);

        toast({
            title: `It's gone..`,
            description: `Deleted ${fileName} from the database`,
        });
    }

    // function abortDownload(fileName: string) {
    //     setActiveDownloads((prev) => ({ ...prev, [fileName]: false }));
    //     setProgress((prev) => ({ ...prev, [fileName]: 0 }));
    //     toast({
    //         title: "Download Aborted",
    //         description: `Download of ${fileName} has been aborted.`,
    //     });
    // }

    useEffect(() => {
        async function getFiles() {
            const response = await fetch(`${backend_url}/api/files`);

            if (!response.ok) return;
            const data: Array<Files> = await response.json();
            setFiles(data);
        }

        getFiles();
    }, []);

    return (
        <Card className="w-full max-w-md p-6 relative">
            <Link className="absolute top-1 left-1" href="/">
                <Button variant="ghost">
                    <ArrowLeft className="mr-2 h-4 w-4" />
                    Back
                </Button>
            </Link>
            <CardHeader className="text-center">
                <CardTitle>Download Module</CardTitle>
                <CardDescription>
                    You can download your files to your default `Downloads`
                    here. <br />
                    <span className="text-red-500">
                        Be careful, as this{" "}
                        <span className="font-bold">WILL OVERRIDE</span> files
                        with the same name and format.
                    </span>
                </CardDescription>
            </CardHeader>
            <CardContent>
                {files && files.length === 0 && (
                    <div className="flex justify-center">No Files.</div>
                )}
                {files && (
                    <div className="space-y-4">
                        {files.map((file, index) => (
                            <DownloadItem
                                key={index}
                                fileName={file.file_name}
                                threadId={file.thread_id}
                                downloadFn={downloadFile}
                                deleteFn={deleteFileFromDb}
                                progress={progress[file.file_name] || 0}
                                isDownloading={
                                    activeDownloads[file.file_name] || false
                                }
                            />
                        ))}
                    </div>
                )}
            </CardContent>
        </Card>
    );
}

function DownloadItem({
    fileName,
    threadId,
    downloadFn,
    deleteFn,
    progress,
    isDownloading,
}: {
    fileName: string;
    threadId: string;
    downloadFn: (fileName: string, threadId: string) => Promise<void>;
    deleteFn: (fileName: string, threadId: string) => Promise<void>;
    progress: number;
    isDownloading: boolean;
}) {
    return (
        <div className="space-y-2">
            <div className="relative">
                <div className="flex items-center justify-between p-3 bg-muted rounded-lg space-x-8">
                    <div className="flex-grow min-w-0 mr-4">
                        <TooltipProvider>
                            <Tooltip>
                                <TooltipTrigger asChild>
                                    <span className="text-inherit font-medium truncate block">
                                        {fileName}
                                    </span>
                                </TooltipTrigger>
                                <TooltipContent>
                                    <p>{fileName}</p>
                                </TooltipContent>
                            </Tooltip>
                        </TooltipProvider>
                    </div>
                    <Button
                        onClick={() => {
                            downloadFn(fileName, threadId);
                        }}
                        disabled={isDownloading}
                    >
                        <Download size={16} className="mr-2" />
                        Download
                    </Button>
                </div>
                <DeleteConfirmationDialog
                    onConfirm={() => deleteFn(fileName, threadId)}
                    fileName={fileName}
                />
            </div>

            {isDownloading && (
                <div className="flex items-center space-x-2">
                    <Progress
                        value={progress}
                        className="flex-grow h-6 transition-all duration-300 ease-in-out"
                    />
                    {/*  <Button
                        variant="destructive"
                        size="icon"
                        className="h-8 w-8"
                        onClick={() => abortFn(fileName)}
                        aria-label="Cancel"
                    >
                        <X size={12} />
                    </Button> */}
                </div>
            )}
        </div>
    );
}
