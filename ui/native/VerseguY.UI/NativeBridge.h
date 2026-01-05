#pragma once

#include <Windows.h>
#include <webview2.h>

void OnWebMessageReceived(_In_ ICoreWebView2* sender, _In_ ICoreWebView2WebMessageReceivedEventArgs* args);
