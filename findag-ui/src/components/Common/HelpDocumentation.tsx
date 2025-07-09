import React, { useState, useCallback, useMemo } from 'react';
import { HelpCircle, Search, Book, Video, Play, ChevronRight, ChevronDown, ExternalLink, Lightbulb } from 'lucide-react';

export interface HelpSection {
  id: string;
  title: string;
  content: string;
  category: string;
  tags: string[];
  videoUrl?: string;
  tutorialSteps?: TutorialStep[];
}

export interface TutorialStep {
  id: string;
  title: string;
  description: string;
  action?: string;
  target?: string;
  completed?: boolean;
}

export interface FAQItem {
  id: string;
  question: string;
  answer: string;
  category: string;
  tags: string[];
}

interface HelpDocumentationProps {
  sections: HelpSection[];
  faqs: FAQItem[];
  onTutorialStepComplete: (tutorialId: string, stepId: string) => void;
  onSearch: (query: string) => void;
  className?: string;
}

export const HelpDocumentation: React.FC<HelpDocumentationProps> = ({
  sections,
  faqs,
  onTutorialStepComplete,
  onSearch,
  className = ""
}) => {
  const [activeTab, setActiveTab] = useState<'help' | 'tutorials' | 'faq' | 'videos' | 'search'>('help');
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedSection, setSelectedSection] = useState<HelpSection | null>(null);
  const [expandedFaqs, setExpandedFaqs] = useState<Set<string>>(new Set());
  const [activeTutorial, setActiveTutorial] = useState<string | null>(null);
  const [completedTutorialSteps, setCompletedTutorialSteps] = useState<Set<string>>(new Set());

  // Categories
  const categories = [
    { id: 'getting-started', name: 'Getting Started', icon: '🚀' },
    { id: 'trading', name: 'Trading', icon: '📈' },
    { id: 'wallet', name: 'Wallet', icon: '💰' },
    { id: 'dag', name: 'DAG', icon: '🔗' },
    { id: 'security', name: 'Security', icon: '🔒' },
    { id: 'troubleshooting', name: 'Troubleshooting', icon: '🔧' }
  ];

  // Filter sections by search query
  const filteredSections = useMemo(() => {
    if (!searchQuery.trim()) return sections;
    
    const query = searchQuery.toLowerCase();
    return sections.filter(section => 
      section.title.toLowerCase().includes(query) ||
      section.content.toLowerCase().includes(query) ||
      section.tags.some(tag => tag.toLowerCase().includes(query))
    );
  }, [sections, searchQuery]);

  // Filter FAQs by search query
  const filteredFaqs = useMemo(() => {
    if (!searchQuery.trim()) return faqs;
    
    const query = searchQuery.toLowerCase();
    return faqs.filter(faq => 
      faq.question.toLowerCase().includes(query) ||
      faq.answer.toLowerCase().includes(query) ||
      faq.tags.some(tag => tag.toLowerCase().includes(query))
    );
  }, [faqs, searchQuery]);

  // Group sections by category
  const sectionsByCategory = useMemo(() => {
    const grouped: Record<string, HelpSection[]> = {};
    filteredSections.forEach(section => {
      if (!grouped[section.category]) {
        grouped[section.category] = [];
      }
      grouped[section.category].push(section);
    });
    return grouped;
  }, [filteredSections]);

  // Group FAQs by category
  const faqsByCategory = useMemo(() => {
    const grouped: Record<string, FAQItem[]> = {};
    filteredFaqs.forEach(faq => {
      if (!grouped[faq.category]) {
        grouped[faq.category] = [];
      }
      grouped[faq.category].push(faq);
    });
    return grouped;
  }, [filteredFaqs]);

  const handleSearch = useCallback((query: string) => {
    setSearchQuery(query);
    onSearch(query);
  }, [onSearch]);

  const toggleFaq = useCallback((faqId: string) => {
    setExpandedFaqs(prev => {
      const newSet = new Set(prev);
      if (newSet.has(faqId)) {
        newSet.delete(faqId);
      } else {
        newSet.add(faqId);
      }
      return newSet;
    });
  }, []);

  const startTutorial = useCallback((tutorialId: string) => {
    setActiveTutorial(tutorialId);
    setActiveTab('tutorials');
  }, []);

  const completeTutorialStep = useCallback((tutorialId: string, stepId: string) => {
    setCompletedTutorialSteps(prev => {
      const newSet = new Set(prev);
      newSet.add(`${tutorialId}-${stepId}`);
      return newSet;
    });
    onTutorialStepComplete(tutorialId, stepId);
  }, [onTutorialStepComplete]);

  const getTutorialProgress = useCallback((tutorial: HelpSection) => {
    if (!tutorial.tutorialSteps) return 0;
    const completed = tutorial.tutorialSteps.filter(step => 
      completedTutorialSteps.has(`${tutorial.id}-${step.id}`)
    ).length;
    return Math.round((completed / tutorial.tutorialSteps.length) * 100);
  }, [completedTutorialSteps]);

  const tabs = [
    { id: 'help', name: 'Help Center', icon: HelpCircle },
    { id: 'tutorials', name: 'Tutorials', icon: Book },
    { id: 'faq', name: 'FAQ', icon: Lightbulb },
    { id: 'videos', name: 'Video Guides', icon: Video },
    { id: 'search', name: 'Search', icon: Search }
  ];

  return (
    <div className={`bg-white dark:bg-gray-800 rounded-lg shadow-lg ${className}`}>
      {/* Header */}
      <div className="border-b border-gray-200 dark:border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <HelpCircle className="h-6 w-6 text-gray-600 dark:text-gray-400" />
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white">Help & Documentation</h2>
          </div>
          <div className="flex items-center space-x-2">
            <button
              onClick={() => window.open('https://docs.findag.com', '_blank')}
              className="flex items-center space-x-1 text-sm text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300"
            >
              <ExternalLink className="h-4 w-4" />
              <span>Full Docs</span>
            </button>
          </div>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200 dark:border-gray-700">
        <nav className="flex space-x-8 px-6">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`py-4 px-1 border-b-2 font-medium text-sm flex items-center space-x-2 ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
            >
              <tab.icon className="h-4 w-4" />
              <span>{tab.name}</span>
            </button>
          ))}
        </nav>
      </div>

      <div className="p-6">
        {/* Help Center Tab */}
        {activeTab === 'help' && (
          <div className="space-y-6">
            {/* Search */}
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => handleSearch(e.target.value)}
                placeholder="Search help articles..."
                className="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
              />
            </div>

            {/* Categories */}
            <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
              {categories.map((category) => (
                <button
                  key={category.id}
                  onClick={() => {
                    setSearchQuery(category.name);
                    handleSearch(category.name);
                  }}
                  className="p-4 border border-gray-200 dark:border-gray-600 rounded-lg hover:border-blue-300 dark:hover:border-blue-600 text-left"
                >
                  <div className="text-2xl mb-2">{category.icon}</div>
                  <h3 className="font-medium text-gray-900 dark:text-white">{category.name}</h3>
                  <p className="text-sm text-gray-500 dark:text-gray-400">
                    {sectionsByCategory[category.id]?.length || 0} articles
                  </p>
                </button>
              ))}
            </div>

            {/* Recent Articles */}
            <div>
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Recent Articles</h3>
              <div className="space-y-3">
                {sections.slice(0, 5).map((section) => (
                  <button
                    key={section.id}
                    onClick={() => setSelectedSection(section)}
                    className="w-full text-left p-3 border border-gray-200 dark:border-gray-600 rounded-lg hover:border-blue-300 dark:hover:border-blue-600"
                  >
                    <h4 className="font-medium text-gray-900 dark:text-white">{section.title}</h4>
                    <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                      {section.content.substring(0, 100)}...
                    </p>
                    <div className="flex items-center space-x-2 mt-2">
                      {section.tags.slice(0, 3).map((tag) => (
                        <span key={tag} className="text-xs bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 px-2 py-1 rounded">
                          {tag}
                        </span>
                      ))}
                    </div>
                  </button>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* Tutorials Tab */}
        {activeTab === 'tutorials' && (
          <div className="space-y-6">
            {activeTutorial ? (
              // Active Tutorial View
              (() => {
                const tutorial = sections.find(s => s.id === activeTutorial);
                if (!tutorial || !tutorial.tutorialSteps) return null;

                return (
                  <div>
                    <div className="flex items-center justify-between mb-4">
                      <button
                        onClick={() => setActiveTutorial(null)}
                        className="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300"
                      >
                        ← Back to Tutorials
                      </button>
                      <div className="text-sm text-gray-500 dark:text-gray-400">
                        {getTutorialProgress(tutorial)}% Complete
                      </div>
                    </div>

                    <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">{tutorial.title}</h3>
                    
                    <div className="space-y-4">
                      {tutorial.tutorialSteps.map((step, index) => {
                        const isCompleted = completedTutorialSteps.has(`${tutorial.id}-${step.id}`);
                        const isActive = index === tutorial.tutorialSteps!.findIndex(s => 
                          !completedTutorialSteps.has(`${tutorial.id}-${s.id}`)
                        );

                        return (
                          <div
                            key={step.id}
                            className={`p-4 border rounded-lg ${
                              isCompleted
                                ? 'border-green-200 dark:border-green-800 bg-green-50 dark:bg-green-900/20'
                                : isActive
                                ? 'border-blue-200 dark:border-blue-800 bg-blue-50 dark:bg-blue-900/20'
                                : 'border-gray-200 dark:border-gray-600'
                            }`}
                          >
                            <div className="flex items-center justify-between mb-2">
                              <h4 className="font-medium text-gray-900 dark:text-white">
                                Step {index + 1}: {step.title}
                              </h4>
                              {isCompleted && (
                                <span className="text-green-600 dark:text-green-400">✓</span>
                              )}
                            </div>
                            <p className="text-sm text-gray-600 dark:text-gray-400 mb-3">
                              {step.description}
                            </p>
                            {isActive && step.action && (
                              <button
                                onClick={() => completeTutorialStep(tutorial.id, step.id)}
                                className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 text-sm"
                              >
                                {step.action}
                              </button>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  </div>
                );
              })()
            ) : (
              // Tutorial List
              <div>
                <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Interactive Tutorials</h3>
                <div className="space-y-4">
                  {sections.filter(s => s.tutorialSteps).map((tutorial) => (
                    <div key={tutorial.id} className="p-4 border border-gray-200 dark:border-gray-600 rounded-lg">
                      <div className="flex items-center justify-between mb-2">
                        <h4 className="font-medium text-gray-900 dark:text-white">{tutorial.title}</h4>
                        <div className="text-sm text-gray-500 dark:text-gray-400">
                          {getTutorialProgress(tutorial)}% Complete
                        </div>
                      </div>
                      <p className="text-sm text-gray-600 dark:text-gray-400 mb-3">
                        {tutorial.content.substring(0, 150)}...
                      </p>
                      <div className="flex items-center space-x-3">
                        <button
                          onClick={() => startTutorial(tutorial.id)}
                          className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 text-sm"
                        >
                          Start Tutorial
                        </button>
                        {tutorial.videoUrl && (
                          <button
                            onClick={() => window.open(tutorial.videoUrl, '_blank')}
                            className="flex items-center space-x-1 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-sm"
                          >
                            <Play className="h-4 w-4" />
                            <span>Watch Video</span>
                          </button>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}

        {/* FAQ Tab */}
        {activeTab === 'faq' && (
          <div className="space-y-6">
            {/* Search */}
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => handleSearch(e.target.value)}
                placeholder="Search FAQ..."
                className="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
              />
            </div>

            {/* FAQ Categories */}
            {Object.entries(faqsByCategory).map(([category, categoryFaqs]) => (
              <div key={category} className="space-y-3">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white">
                  {categories.find(c => c.id === category)?.name || category}
                </h3>
                <div className="space-y-2">
                  {categoryFaqs.map((faq) => (
                    <div key={faq.id} className="border border-gray-200 dark:border-gray-600 rounded-lg">
                      <button
                        onClick={() => toggleFaq(faq.id)}
                        className="w-full p-4 text-left flex items-center justify-between hover:bg-gray-50 dark:hover:bg-gray-700"
                      >
                        <span className="font-medium text-gray-900 dark:text-white">{faq.question}</span>
                        {expandedFaqs.has(faq.id) ? (
                          <ChevronDown className="h-4 w-4 text-gray-500" />
                        ) : (
                          <ChevronRight className="h-4 w-4 text-gray-500" />
                        )}
                      </button>
                      {expandedFaqs.has(faq.id) && (
                        <div className="px-4 pb-4">
                          <p className="text-sm text-gray-600 dark:text-gray-400">{faq.answer}</p>
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Video Guides Tab */}
        {activeTab === 'videos' && (
          <div className="space-y-6">
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">Video Guides</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {sections.filter(s => s.videoUrl).map((section) => (
                <div key={section.id} className="border border-gray-200 dark:border-gray-600 rounded-lg overflow-hidden">
                  <div className="aspect-video bg-gray-100 dark:bg-gray-700 flex items-center justify-center">
                    <button
                      onClick={() => window.open(section.videoUrl, '_blank')}
                      className="flex items-center space-x-2 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300"
                    >
                      <Play className="h-8 w-8" />
                      <span>Watch Video</span>
                    </button>
                  </div>
                  <div className="p-4">
                    <h4 className="font-medium text-gray-900 dark:text-white mb-2">{section.title}</h4>
                    <p className="text-sm text-gray-600 dark:text-gray-400">{section.content.substring(0, 100)}...</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Search Tab */}
        {activeTab === 'search' && (
          <div className="space-y-6">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => handleSearch(e.target.value)}
                placeholder="Search documentation..."
                className="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
              />
            </div>

            {searchQuery && (
              <div className="space-y-4">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white">Search Results</h3>
                
                {/* Help Articles */}
                {filteredSections.length > 0 && (
                  <div>
                    <h4 className="font-medium text-gray-700 dark:text-gray-300 mb-2">Help Articles</h4>
                    <div className="space-y-2">
                      {filteredSections.map((section) => (
                        <button
                          key={section.id}
                          onClick={() => setSelectedSection(section)}
                          className="w-full text-left p-3 border border-gray-200 dark:border-gray-600 rounded-lg hover:border-blue-300 dark:hover:border-blue-600"
                        >
                          <h5 className="font-medium text-gray-900 dark:text-white">{section.title}</h5>
                          <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                            {section.content.substring(0, 100)}...
                          </p>
                        </button>
                      ))}
                    </div>
                  </div>
                )}

                {/* FAQ Results */}
                {filteredFaqs.length > 0 && (
                  <div>
                    <h4 className="font-medium text-gray-700 dark:text-gray-300 mb-2">FAQ</h4>
                    <div className="space-y-2">
                      {filteredFaqs.map((faq) => (
                        <div key={faq.id} className="p-3 border border-gray-200 dark:border-gray-600 rounded-lg">
                          <h5 className="font-medium text-gray-900 dark:text-white">{faq.question}</h5>
                          <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">{faq.answer}</p>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {filteredSections.length === 0 && filteredFaqs.length === 0 && (
                  <p className="text-gray-500 dark:text-gray-400 text-center py-8">
                    No results found for "{searchQuery}"
                  </p>
                )}
              </div>
            )}
          </div>
        )}

        {/* Selected Section Modal */}
        {selectedSection && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto">
              <div className="p-6">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-lg font-medium text-gray-900 dark:text-white">{selectedSection.title}</h3>
                  <button
                    onClick={() => setSelectedSection(null)}
                    className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                  >
                    ✕
                  </button>
                </div>
                <div className="prose dark:prose-invert max-w-none">
                  <p className="text-gray-600 dark:text-gray-400">{selectedSection.content}</p>
                </div>
                {selectedSection.videoUrl && (
                  <div className="mt-4">
                    <button
                      onClick={() => window.open(selectedSection.videoUrl, '_blank')}
                      className="flex items-center space-x-2 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300"
                    >
                      <Play className="h-4 w-4" />
                      <span>Watch Video Guide</span>
                    </button>
                  </div>
                )}
                {selectedSection.tutorialSteps && (
                  <div className="mt-4">
                    <button
                      onClick={() => {
                        setSelectedSection(null);
                        startTutorial(selectedSection.id);
                      }}
                      className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
                    >
                      Start Interactive Tutorial
                    </button>
                  </div>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}; 