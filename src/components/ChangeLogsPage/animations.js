export const createAnimationStyles = () => {
	const style = document.createElement('style');
	style.textContent = `
		@keyframes slideInFromSide {
			from {
				opacity: 0;
				transform: translateX(-30px);
			}
			to {
				opacity: 1;
				transform: translateX(0);
			}
		}
		
		@keyframes slideInFromContent {
			from {
				opacity: 0;
				transform: translateX(30px) scale(0.9);
			}
			to {
				opacity: 1;
				transform: translateX(0) scale(1);
			}
		}
		
		@keyframes bounceIn {
			0% {
				opacity: 0;
				transform: scale(0.3) rotate(-180deg);
			}
			50% {
				opacity: 0.8;
				transform: scale(1.1) rotate(-90deg);
			}
			100% {
				opacity: 1;
				transform: scale(1) rotate(0deg);
			}
		}
		
		@keyframes slideDown {
			from {
				opacity: 0;
				height: 0;
			}
			to {
				opacity: 1;
				height: 100%;
			}
		}
		
		@keyframes pulse {
			0%, 100% {
				opacity: 0.3;
			}
			50% {
				opacity: 0.8;
			}
		}
		
		@keyframes spin {
			from {
				transform: rotate(0deg) scale(1.2);
			}
			to {
				transform: rotate(360deg) scale(1.2);
			}
		}
		
		@keyframes wiggle {
			0%, 100% {
				transform: rotate(0deg);
			}
			25% {
				transform: rotate(-3deg) scale(1.05);
			}
			75% {
				transform: rotate(3deg) scale(1.05);
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
	`;
	return style;
};
