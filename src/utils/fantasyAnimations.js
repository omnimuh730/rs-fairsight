import { useEffect } from 'react';

// Animation keyframes
export const ANIMATION_KEYFRAMES = `
  @keyframes fadeInUp {
    from {
      opacity: 0;
      transform: translateY(30px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  @keyframes fadeInDown {
    from {
      opacity: 0;
      transform: translateY(-30px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  @keyframes slideInLeft {
    from {
      opacity: 0;
      transform: translateX(-50px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
  
  @keyframes slideInRight {
    from {
      opacity: 0;
      transform: translateX(50px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
  
  @keyframes scaleIn {
    from {
      opacity: 0;
      transform: scale(0.8);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
  
  @keyframes rotateIn {
    from {
      opacity: 0;
      transform: rotate(-180deg) scale(0.5);
    }
    to {
      opacity: 1;
      transform: rotate(0deg) scale(1);
    }
  }
  
  @keyframes pulse {
    0%, 100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.05);
    }
  }
  
  @keyframes shimmer {
    0% {
      background-position: -200px 0;
    }
    100% {
      background-position: calc(200px + 100%) 0;
    }
  }
  
  @keyframes float {
    0%, 100% {
      transform: translateY(0px);
    }
    50% {
      transform: translateY(-10px);
    }
  }
  
  @keyframes bounce {
    0%, 20%, 53%, 80%, 100% {
      animation-timing-function: cubic-bezier(0.215, 0.61, 0.355, 1);
      transform: translate3d(0, 0, 0);
    }
    40%, 43% {
      animation-timing-function: cubic-bezier(0.755, 0.05, 0.855, 0.06);
      transform: translate3d(0, -15px, 0);
    }
    70% {
      animation-timing-function: cubic-bezier(0.755, 0.05, 0.855, 0.06);
      transform: translate3d(0, -7px, 0);
    }
    90% {
      transform: translate3d(0, -2px, 0);
    }
  }
  
  @keyframes gradientShift {
    0% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
    100% {
      background-position: 0% 50%;
    }
  }
  
  @keyframes wiggle {
    0%, 100% {
      transform: rotate(0deg);
    }
    25% {
      transform: rotate(-3deg) scale(1.02);
    }
    75% {
      transform: rotate(3deg) scale(1.02);
    }
  }
  
  @keyframes sparkle {
    0% {
      opacity: 0;
      transform: scale(0) rotate(0deg);
    }
    50% {
      opacity: 1;
      transform: scale(1.2) rotate(180deg);
    }
    100% {
      opacity: 0;
      transform: scale(0) rotate(360deg);
    }
  }
`;

// Hook to inject animations
export const useAnimations = () => {
  useEffect(() => {
    const styleId = 'fantasy-animations';
    if (!document.getElementById(styleId)) {
      const style = document.createElement('style');
      style.id = styleId;
      style.textContent = ANIMATION_KEYFRAMES;
      document.head.appendChild(style);
    }

    return () => {
      const existingStyle = document.getElementById(styleId);
      if (existingStyle) {
        document.head.removeChild(existingStyle);
      }
    };
  }, []);
};

// Predefined animation styles
export const animationStyles = {
  fadeInUp: (delay = 0) => ({
    animation: `fadeInUp 0.8s ease-out ${delay}s both`,
  }),
  
  fadeInDown: (delay = 0) => ({
    animation: `fadeInDown 0.8s ease-out ${delay}s both`,
  }),
  
  slideInLeft: (delay = 0) => ({
    animation: `slideInLeft 0.8s ease-out ${delay}s both`,
  }),
  
  slideInRight: (delay = 0) => ({
    animation: `slideInRight 0.8s ease-out ${delay}s both`,
  }),
  
  scaleIn: (delay = 0) => ({
    animation: `scaleIn 0.6s ease-out ${delay}s both`,
  }),
  
  rotateIn: (delay = 0) => ({
    animation: `rotateIn 1s ease-out ${delay}s both`,
  }),
  
  floatingCard: {
    animation: 'float 3s ease-in-out infinite',
    transition: 'all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1)',
    '&:hover': {
      transform: 'translateY(-8px) scale(1.02)',
      boxShadow: '0 15px 40px rgba(0,0,0,0.2)',
      animation: 'pulse 0.8s ease-in-out',
    },
  },
  
  glassMorphism: {
    background: 'linear-gradient(145deg, rgba(255,255,255,0.1), rgba(255,255,255,0.05))',
    backdropFilter: 'blur(10px)',
    border: '1px solid rgba(255,255,255,0.2)',
    borderRadius: '16px',
  },
  
  gradientText: (colors = ['#2196F3', '#21CBF3']) => ({
    background: `linear-gradient(45deg, ${colors.join(', ')})`,
    backgroundSize: '400% 400%',
    WebkitBackgroundClip: 'text',
    WebkitTextFillColor: 'transparent',
    animation: 'gradientShift 3s ease infinite',
  }),
  
  shimmering: {
    background: 'linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%)',
    backgroundSize: '200px 100%',
    animation: 'shimmer 2s infinite',
  },
  
  bounceIn: (delay = 0) => ({
    animation: `bounce 1s ease ${delay}s both`,
  }),
  
  interactiveButton: {
    transition: 'all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1)',
    '&:hover': {
      transform: 'translateY(-2px)',
      boxShadow: '0 8px 25px rgba(0,0,0,0.15)',
      animation: 'wiggle 0.5s ease-in-out',
    },
    '&:active': {
      transform: 'translateY(0)',
      animation: 'pulse 0.3s ease-in-out',
    },
  },
  
  staggeredList: (index) => ({
    ...animationStyles.fadeInUp(index * 0.1),
    '&:hover': {
      transform: 'translateX(5px)',
      transition: 'transform 0.3s ease',
      '&::before': {
        content: '"✨"',
        position: 'absolute',
        left: '-25px',
        animation: 'sparkle 0.6s ease-out',
      },
    },
  }),
};

// Color schemes for different themes
export const colorSchemes = {
  primary: {
    main: '#2196F3',
    light: '#64B5F6',
    dark: '#1976D2',
    gradient: ['#2196F3', '#21CBF3'],
  },
  secondary: {
    main: '#9C27B0',
    light: '#BA68C8',
    dark: '#7B1FA2',
    gradient: ['#9C27B0', '#E1BEE7'],
  },
  success: {
    main: '#4CAF50',
    light: '#81C784',
    dark: '#388E3C',
    gradient: ['#4CAF50', '#81C784'],
  },
  warning: {
    main: '#FF9800',
    light: '#FFB74D',
    dark: '#F57C00',
    gradient: ['#FF9800', '#FFB74D'],
  },
  info: {
    main: '#00BCD4',
    light: '#4DD0E1',
    dark: '#0097A7',
    gradient: ['#00BCD4', '#4DD0E1'],
  },
};

export default {
  useAnimations,
  animationStyles,
  colorSchemes,
  ANIMATION_KEYFRAMES,
};
