﻿<?xml version="1.0" encoding="utf-8"?>
<Project ToolsVersion="3.5" DefaultTargets="Build" xmlns="http://schemas.microsoft.com/developer/msbuild/2003">
  <PropertyGroup>
    <Configuration Condition=" '$(Configuration)' == '' ">Debug</Configuration>
    <Platform Condition=" '$(Platform)' == '' ">AnyCPU</Platform>
    <ProductVersion>8.0.30703</ProductVersion>
    <SchemaVersion>2.0</SchemaVersion>
    <ProjectGuid>dcf30d18-70c2-49ae-953d-65a9dc5dd205</ProjectGuid>
    <OutputType>library</OutputType>
    <RootNamespace>DirectX2</RootNamespace>
    <AssemblyName>DirectX2</AssemblyName>
    <TargetFrameworkVersion>v3.5</TargetFrameworkVersion>
    <FileAlignment>512</FileAlignment>
    <Name>3DVisualization</Name>
  </PropertyGroup>
  <PropertyGroup Condition=" '$(Configuration)|$(Platform)' == 'Debug|AnyCPU' ">
    <DebugSymbols>true</DebugSymbols>
    <DebugType>full</DebugType>
    <Optimize>false</Optimize>
    <OutputPath>bin\Debug\</OutputPath>
    <DefineConstants>DEBUG;TRACE</DefineConstants>
    <ErrorReport>prompt</ErrorReport>
    <WarningLevel>3</WarningLevel>
  </PropertyGroup>
  <PropertyGroup Condition=" '$(Configuration)|$(Platform)' == 'Release|AnyCPU' ">
    <DebugType>pdbonly</DebugType>
    <Optimize>true</Optimize>
    <OutputPath>bin\Release\</OutputPath>
    <DefineConstants>TRACE</DefineConstants>
    <ErrorReport>prompt</ErrorReport>
    <WarningLevel>3</WarningLevel>
  </PropertyGroup>
  <ItemGroup>
    <Compile Include="dxlib.fs" />
    <None Include="Script.fsx" />
  </ItemGroup>
  <ItemGroup>
    <Reference Include="Microsoft.DirectX, Version=1.0.2902.0, Culture=neutral, PublicKeyToken=31bf3856ad364e35">
      <Name>Microsoft.DirectX</Name>
      <AssemblyName>Microsoft.DirectX.dll</AssemblyName>
    </Reference>
    <Reference Include="Microsoft.DirectX.Direct3D, Version=1.0.2902.0, Culture=neutral, PublicKeyToken=31bf3856ad364e35">
      <Name>Microsoft.DirectX.Direct3D</Name>
      <AssemblyName>Microsoft.DirectX.Direct3D.dll</AssemblyName>
    </Reference>
    <Reference Include="Microsoft.DirectX.Direct3DX, Version=1.0.2911.0, Culture=neutral, PublicKeyToken=31bf3856ad364e35">
      <Name>Microsoft.DirectX.Direct3DX</Name>
      <AssemblyName>Microsoft.DirectX.Direct3DX.dll</AssemblyName>
    </Reference>
    <Reference Include="System" />
    <Reference Include="System.Core">
      <RequiredTargetFramework>3.5</RequiredTargetFramework>
    </Reference>
    <Reference Include="System.Drawing" />
    <Reference Include="System.Windows.Forms" />
  </ItemGroup>
  <Import Project="$(MSBuildExtensionsPath)\FSharp\1.0\Microsoft.FSharp.Targets" />
  <!-- To modify your build process, add your task inside one of the targets below and uncomment it. 
	     Other similar extension points exist, see Microsoft.Common.targets.
	<Target Name="BeforeBuild">
	</Target>
	<Target Name="AfterBuild">
	</Target>
	-->
</Project>