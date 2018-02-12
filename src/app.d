/*
 * Copyright 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software
 * and associated documentation files (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute,
 * sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all copies or
 * substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
 * BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

import std.experimental.logger;
import std.stdio;
import gl;
import draw;
import configuration;
import litecraft;
import resource_manager;
import scenes;

void main() {
	// Litecraft logo
	writeln(r"  _    _ _                    __ _   
 | |  (_) |_ ___ __ _ _ __ _ / _| |_ 
 | |__| |  _/ -_) _| '_/ _` |  _|  _|
 |____|_|\__\___\__|_| \__,_|_|  \__|
                                     ");

	try {
		auto configuration = new SDLConfigurationAdapter;
		auto litecraft = new Litecraft(configuration);
		
		litecraft.scene = new LoadingScene;

		/*******************
		* Create load queue
		********************/

		// Shaders
		new Shader("litecraft").loadResource;
		new Shader("fullquad").loadResource;
		new Shader("block").loadResource;
		new Shader("quad").loadResource;
		new Shader("noise").loadResource;
		new Shader("blur").loadResource;

		// Primitives
		new Quad().loadResource;
		new FullScreenQuad().loadResource;
		new TexturedFullScreenQuad().loadResource;

		// Textures
		new Texture("logo", "litecraft").loadResource;

		new Texture("menu_1", "litecraft").loadResource;
		new Texture("menu_2", "litecraft").loadResource;
		new Texture("menu_3", "litecraft").loadResource;
		new Texture("menu_4", "litecraft").loadResource;
		new Texture("menu_5", "litecraft").loadResource;
		new Texture("menu_6", "litecraft").loadResource;
		new Texture("menu_7", "litecraft").loadResource;

		load();
		configuration.save();
	}
	catch (Exception e) {
		infof("Fatal error: %s\n%s", e.toString, e.info);
	}
}
