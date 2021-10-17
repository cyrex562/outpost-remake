#pragma once

#include "Core/Window.h"
#include "Core/Button.h"
#include "Core/TextField.h"
#include "Core/ListBox.h"

#include <NAS2D/Signal/Signal.h>
#include <NAS2D/EventHandler.h>


class FileIo : public Window
{
public:
	enum class FileOperation
	{
		Load,
		Save
	};

	using FileOperationSignal = NAS2D::Signal<const std::string&, FileOperation>;

	FileIo();
	~FileIo() override;

	void setMode(FileOperation fileOp);
	void scanDirectory(const std::string& directory);

	FileOperationSignal::Source& fileOperation() { return mSignal; }

	void update() override;

protected:
	void onDoubleClick(NAS2D::EventHandler::MouseButton button, int x, int y);
	void onKeyDown(NAS2D::EventHandler::KeyCode key, NAS2D::EventHandler::KeyModifier mod, bool repeat);

private:
	void onClose();
	void onFileIo();
	void onFileDelete();

	void onFileSelect();
	void onFileNameChange(TextControl* control);

	FileOperationSignal mSignal;

	FileOperation mMode;

	Button btnClose;
	Button btnFileOp;
	Button btnFileDelete;

	TextField txtFileName;

	ListBox<> mListBox;
};
